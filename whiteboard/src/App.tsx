import { useCallback, useEffect, useMemo, useRef, useState } from 'react'
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
  // Recadrer le contenu à la vue (canvas partagé : tout le monde voit tout,
  // quelle que soit la largeur de son panneau — cf. tableau blanc redimensionnable).
  | { type: 'fit-content' }

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
//
// Inclut désormais les champs de VIEWPORT (scrollX/scrollY/zoom/width/height…) :
// le cadrage est géré localement par fitToView() pour que chaque client voie tout
// le contenu indépendamment de la largeur de son panneau. Synchroniser le viewport
// entre clients de largeurs différentes provoquait justement le débordement signalé.

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
  // Viewport — propre à chaque client (largeur de panneau différente).
  'scrollX',
  'scrollY',
  'zoom',
  'width',
  'height',
  'offsetTop',
  'offsetLeft',
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
  // Horodatage de la dernière édition LOCALE : évite de recadrer (et donc de
  // « bouger la vue ») pendant que l'utilisateur est en train de dessiner.
  const dernierEditLocalRef = useRef<number>(0)

  useEffect(() => {
    apiRef.current = excalidrawAPI
  }, [excalidrawAPI])

  useEffect(() => {
    if (!excalidrawAPI) return
    window.parent.postMessage({ type: 'excalidraw-ready' }, '*')
  }, [excalidrawAPI])

  // ── Cadrage « tout le contenu visible » (canvas partagé) ───────────────────
  // Calcule la boîte englobante de tous les éléments et ajuste scroll + zoom pour
  // que tout tienne dans le viewport, SANS jamais zoomer au-delà de 100 %.
  const recadrerSurContenu = useCallback((force: boolean) => {
    const api = apiRef.current
    if (!api) return
    if (!force && Date.now() - dernierEditLocalRef.current < 1500) return

    const elements = api.getSceneElements().filter((e) => !e.isDeleted)
    if (elements.length === 0) return

    let minX = Infinity
    let minY = Infinity
    let maxX = -Infinity
    let maxY = -Infinity
    for (const el of elements) {
      minX = Math.min(minX, el.x)
      minY = Math.min(minY, el.y)
      maxX = Math.max(maxX, el.x + el.width)
      maxY = Math.max(maxY, el.y + el.height)
    }
    if (!Number.isFinite(minX) || !Number.isFinite(maxX)) return

    const marge = 48
    const contenuL = (maxX - minX) + marge * 2
    const contenuH = (maxY - minY) + marge * 2

    const etat = api.getAppState() as unknown as { width: number; height: number }
    const vw = etat.width
    const vh = etat.height
    if (!vw || !vh) return

    // Zoom = ajuster pour tout voir, mais jamais au-delà de 100 %.
    const zoom = Math.max(0.1, Math.min(vw / contenuL, vh / contenuH, 1))
    const centreX = (minX + maxX) / 2
    const centreY = (minY + maxY) / 2
    // Convention Excalidraw : screen = (scene + scroll) * zoom (offset 0).
    const scrollX = vw / (2 * zoom) - centreX
    const scrollY = vh / (2 * zoom) - centreY

    // remoteRef évite que ce changement de viewport ne soit rediffusé en boucle.
    remoteRef.current = true
    api.updateScene({
      appState: { scrollX, scrollY, zoom: { value: zoom } } as never,
    })
  }, [])

  // Recadrage « doux » (arrivée d'un dessin distant — respecte l'édition locale)
  // et « ferme » (redimensionnement / chargement / bouton — force le recadrage).
  const recadrerDoux = useMemo(() => debounce(() => recadrerSurContenu(false), 250), [recadrerSurContenu])
  const recadrerFerme = useMemo(() => debounce(() => recadrerSurContenu(true), 180), [recadrerSurContenu])

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

      // Édition locale : mémoriser l'instant (pour ne pas recadrer pendant le dessin).
      dernierEditLocalRef.current = Date.now()
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
          // Le contenu distant peut sortir du champ de vision (panneaux de
          // largeurs différentes) → recadrer pour tout garder visible.
          recadrerDoux()
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
          // Recadrer après chargement initial (laisser la scène se poser).
          setTimeout(() => recadrerSurContenu(true), 60)
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

        case 'fit-content': {
          recadrerSurContenu(true)
          break
        }

        default:
          break
      }
    }

    window.addEventListener('message', handler)
    return () => window.removeEventListener('message', handler)
  }, [recadrerDoux, recadrerSurContenu])

  // Redimensionnement de l'iframe (le panneau du tableau blanc est redimensionnable) :
  // recadrer pour que le contenu reste entièrement visible dans la nouvelle largeur.
  useEffect(() => {
    const onResize = () => recadrerFerme()
    window.addEventListener('resize', onResize)
    return () => window.removeEventListener('resize', onResize)
  }, [recadrerFerme])

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
