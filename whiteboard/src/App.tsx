import { useEffect, useCallback, useRef } from 'react'
import { Tldraw, getSnapshot, loadSnapshot, Editor } from 'tldraw'
import 'tldraw/tldraw.css'

export default function WhiteboardApp() {
  const editorRef = useRef<Editor | null>(null)

  // Envoyer les operations locales au parent Vue
  const handleMount = useCallback((editor: Editor) => {
    editorRef.current = editor

    // Ecouter les changements utilisateur pour les diffuser aux autres participants
    editor.store.listen(
      (entry) => {
        window.parent.postMessage(
          {
            type: 'tldraw-operation',
            payload: entry.changes,
          },
          '*',
        )
      },
      { source: 'user', scope: 'document' },
    )
  }, [])

  // Recevoir les messages du parent Vue
  useEffect(() => {
    function handleMessage(event: MessageEvent) {
      const editor = editorRef.current
      if (!editor) return

      switch (event.data?.type) {
        case 'apply-operation':
          // Appliquer les changements distants (depuis DataChannel LiveKit)
          editor.store.mergeRemoteChanges(() => {
            const changes = event.data.payload
            if (changes.added) {
              for (const record of Object.values(changes.added)) {
                editor.store.put([record as any])
              }
            }
            if (changes.updated) {
              for (const [, to] of Object.values(changes.updated) as any[]) {
                editor.store.put([to])
              }
            }
            if (changes.removed) {
              const ids = Object.keys(changes.removed)
              if (ids.length > 0) {
                editor.store.remove(ids as any[])
              }
            }
          })
          break

        case 'load-snapshot':
          // Charger un snapshot depuis le backend
          if (event.data.snapshot) {
            loadSnapshot(editor.store, event.data.snapshot)
          }
          break

        case 'get-snapshot':
          // Renvoyer le snapshot actuel pour sauvegarde periodique
          window.parent.postMessage(
            {
              type: 'tldraw-snapshot',
              payload: getSnapshot(editor.store),
            },
            '*',
          )
          break

        case 'clear':
          // Effacer tout le contenu
          editor.selectAll()
          editor.deleteShapes(editor.getSelectedShapeIds())
          break
      }
    }

    window.addEventListener('message', handleMessage)
    return () => window.removeEventListener('message', handleMessage)
  }, [])

  return (
    <div style={{ width: '100%', height: '100vh' }}>
      <Tldraw onMount={handleMount} />
    </div>
  )
}
