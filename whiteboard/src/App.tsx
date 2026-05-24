import { useCallback, useEffect, useRef, useState } from 'react'
import { Excalidraw } from '@excalidraw/excalidraw'
import type {
  ExcalidrawImperativeAPI,
  BinaryFileData,
  BinaryFiles,
  AppState,
} from '@excalidraw/excalidraw/types'
import type { ExcalidrawElement } from '@excalidraw/excalidraw/element/types'
import '@excalidraw/excalidraw/index.css'

// ─── Types des messages postMessage (cf. contracts/postmessage.md) ───────────

type MessageFromParent =
  | { type: 'apply-operation'; payload: { elements: ExcalidrawElement[]; appState?: Record<string, unknown> } }
  | { type: 'load-snapshot'; snapshot: unknown }
  | { type: 'get-snapshot' }
  | { type: 'clear' }

interface SnapshotSortie {
  elements: readonly ExcalidrawElement[]
  appState: Record<string, unknown>
  files: BinaryFiles
}

// ─── Lecture défensive des snapshots (data-model.md) ─────────────────────────

function estSnapshotExcalidrawValide(donnees: unknown): boolean {
  if (donnees === null || donnees === undefined) return false
  if (typeof donnees !== 'object') return false
  const obj = donnees as Record<string, unknown>
  if ('document' in obj || 'store' in obj || 'records' in obj) return false
  if (!Array.isArray(obj.elements)) return false
  return true
}

// ─── Filtre des champs volatils d'appState (data-model.md) ───────────────────

const CHAMPS_VOLATILS = new Set([
  'collaborators',
  'selectedElementIds',
  'selectedGroupIds',
  'editingElement',
  'draggingElement',
  'resizingElement',
  'cursorButton',
  'scrolledOutside',
  'contextMenu',
  'openPopup',
  'openMenu',
  'openDialog',
  'openSidebar',
  'toast',
  'errorMessage',
  'showHyperlinkPopup',
  'showWelcomeScreen',
])

function filterAppState(appState: Record<string, unknown>): Record<string, unknown> {
  const filtre: Record<string, unknown> = {}
  for (const cle of Object.keys(appState)) {
    if (!CHAMPS_VOLATILS.has(cle)) {
      filtre[cle] = appState[cle]
    }
  }
  return filtre
}

// ─── Validation image locale (FR-001a) ───────────────────────────────────────

const FORMATS_IMAGE_AUTORISES = new Set(['image/jpeg', 'image/png'])
const TAILLE_IMAGE_MAX_OCTETS = 2 * 1024 * 1024

function imageEstAcceptable(mimeType: string, tailleOctets: number): 'ok' | 'format' | 'taille' {
  if (!FORMATS_IMAGE_AUTORISES.has(mimeType)) return 'format'
  if (tailleOctets > TAILLE_IMAGE_MAX_OCTETS) return 'taille'
  return 'ok'
}

function estimerTailleDataURL(dataURL: string): number {
  const virgule = dataURL.indexOf(',')
  if (virgule < 0) return dataURL.length
  const b64 = dataURL.slice(virgule + 1)
  return Math.floor((b64.length * 3) / 4)
}

// ─── Débouncing simple ───────────────────────────────────────────────────────

function debounce<A extends unknown[]>(fn: (...args: A) => void, delai: number): (...args: A) => void {
  let timer: ReturnType<typeof setTimeout> | null = null
  return (...args: A) => {
    if (timer) clearTimeout(timer)
    timer = setTimeout(() => fn(...args), delai)
  }
}

// ─── Composant principal ─────────────────────────────────────────────────────

export default function WhiteboardApp() {
  const [excalidrawAPI, setExcalidrawAPI] = useState<ExcalidrawImperativeAPI | null>(null)
  const apiRef = useRef<ExcalidrawImperativeAPI | null>(null)
  const remoteRef = useRef<boolean>(false)
  const fichiersConnusRef = useRef<Set<string>>(new Set())
  const fichiersRejetesRef = useRef<Set<string>>(new Set())

  useEffect(() => {
    apiRef.current = excalidrawAPI
  }, [excalidrawAPI])

  useEffect(() => {
    if (!excalidrawAPI) return
    window.parent.postMessage({ type: 'excalidraw-ready' }, '*')
  }, [excalidrawAPI])

  const diffuserOperation = useRef(
    debounce((elements: readonly ExcalidrawElement[], appState: Record<string, unknown>) => {
      window.parent.postMessage(
        {
          type: 'excalidraw-operation',
          payload: {
            elements,
            appState: filterAppState(appState),
          },
        },
        '*',
      )
    }, 80),
  ).current

  const handleChange = useCallback(
    (elements: readonly ExcalidrawElement[], appState: AppState, files: BinaryFiles) => {
      const api = apiRef.current
      if (!api) return

      const fichiersARetirer: string[] = []
      for (const [id, fichier] of Object.entries(files) as Array<[string, BinaryFileData]>) {
        if (fichiersConnusRef.current.has(id) || fichiersRejetesRef.current.has(id)) continue
        fichiersConnusRef.current.add(id)

        const taille = estimerTailleDataURL(fichier.dataURL)
        const verdict = imageEstAcceptable(fichier.mimeType, taille)
        if (verdict !== 'ok') {
          fichiersRejetesRef.current.add(id)
          fichiersConnusRef.current.delete(id)
          fichiersARetirer.push(id)
          window.parent.postMessage(
            {
              type: 'excalidraw-image-rejected',
              payload: {
                raison: verdict,
                nomFichier: (fichier as { name?: string }).name ?? id,
              },
            },
            '*',
          )
        }
      }

      if (fichiersARetirer.length > 0) {
        const rejetSet = new Set(fichiersARetirer)
        const elementsNettoyes = elements.filter((el) => {
          if (el.type !== 'image') return true
          const fileId = (el as { fileId?: string }).fileId
          return !fileId || !rejetSet.has(fileId)
        })
        if (elementsNettoyes.length !== elements.length) {
          remoteRef.current = true
          api.updateScene({ elements: elementsNettoyes })
        }
      }

      if (remoteRef.current) {
        remoteRef.current = false
        return
      }

      diffuserOperation(elements, appState as unknown as Record<string, unknown>)
    },
    [diffuserOperation],
  )

  useEffect(() => {
    function handler(event: MessageEvent<MessageFromParent>) {
      const api = apiRef.current
      if (!api) return
      const data = event.data
      if (!data || typeof data !== 'object') return

      switch (data.type) {
        case 'apply-operation': {
          const { elements, appState } = data.payload ?? { elements: [] as ExcalidrawElement[] }
          if (!Array.isArray(elements)) return
          remoteRef.current = true
          api.updateScene({
            elements: elements as ExcalidrawElement[],
            ...(appState ? { appState: appState as never } : {}),
          })
          break
        }

        case 'load-snapshot': {
          if (!estSnapshotExcalidrawValide(data.snapshot)) {
            remoteRef.current = true
            api.updateScene({ elements: [], appState: {} as never })
            break
          }
          const snap = data.snapshot as {
            elements: ExcalidrawElement[]
            appState?: Record<string, unknown>
            files?: BinaryFiles
          }
          remoteRef.current = true
          api.updateScene({
            elements: snap.elements,
            ...(snap.appState ? { appState: snap.appState as never } : {}),
          })
          if (snap.files) {
            const fichiers = Object.values(snap.files) as BinaryFileData[]
            if (fichiers.length > 0) {
              api.addFiles(fichiers)
              for (const f of fichiers) fichiersConnusRef.current.add(f.id)
            }
          }
          break
        }

        case 'get-snapshot': {
          const payload: SnapshotSortie = {
            elements: api.getSceneElements(),
            appState: filterAppState(api.getAppState() as unknown as Record<string, unknown>),
            files: api.getFiles() ?? {},
          }
          window.parent.postMessage({ type: 'excalidraw-snapshot', payload }, '*')
          break
        }

        case 'clear': {
          remoteRef.current = true
          api.resetScene()
          fichiersConnusRef.current.clear()
          fichiersRejetesRef.current.clear()
          break
        }

        default:
          break
      }
    }

    window.addEventListener('message', handler)
    return () => window.removeEventListener('message', handler)
  }, [])

  return (
    <div style={{ width: '100%', height: '100vh' }}>
      <Excalidraw
        excalidrawAPI={setExcalidrawAPI}
        langCode="fr-FR"
        onChange={handleChange}
        UIOptions={{ dockedSidebarBreakpoint: 0 }}
      />
    </div>
  )
}
