import type { OutputData, API } from '@editorjs/editorjs'

// Types pour les blocs Editor.js
export interface EditorJsBlock {
  id?: string
  type: string
  data: Record<string, unknown>
}

export interface EditorJsData {
  time?: number
  blocks: EditorJsBlock[]
  version?: string
}

// Configuration des outils disponibles
export type EditorJsTools =
  | 'header'
  | 'list'
  | 'paragraph'
  | 'image'
  | 'quote'
  | 'code'
  | 'delimiter'
  | 'embed'
  | 'table'
  | 'link'
  | 'marker'
  | 'inlineCode'
  | 'underline'

export interface UseEditorJsOptions {
  /** ID de l'élément conteneur */
  holderId?: string
  /** Placeholder du paragraphe vide */
  placeholder?: string
  /** Données initiales */
  data?: EditorJsData
  /** Outils à activer (tous par défaut) */
  tools?: EditorJsTools[]
  /** Mode lecture seule */
  readOnly?: boolean
  /** Callback lors des changements */
  onChange?: (data: EditorJsData) => void
  /** URL de l'endpoint pour upload d'images */
  imageUploadUrl?: string
  /** Autofocus à l'initialisation */
  autofocus?: boolean
  /** Configuration des niveaux de header */
  headerLevels?: number[]
}

const DEFAULT_TOOLS: EditorJsTools[] = [
  'header',
  'list',
  'paragraph',
  'quote',
  'code',
  'delimiter',
  'embed',
  'table',
  'link',
  'marker',
  'inlineCode',
  'underline'
]

/**
 * Composable pour gérer une instance Editor.js
 *
 * @example
 * ```vue
 * <script setup>
 * const { editorRef, isReady, save, clear } = useEditorJs({
 *   holderId: 'editor',
 *   placeholder: 'Commencez à écrire...',
 *   onChange: (data) => console.log(data)
 * })
 * </script>
 * ```
 */
export function useEditorJs(options: UseEditorJsOptions = {}) {
  const {
    holderId = 'editorjs',
    placeholder = 'Commencez à écrire votre contenu...',
    data,
    tools = DEFAULT_TOOLS,
    readOnly = false,
    onChange,
    imageUploadUrl,
    autofocus = false,
    headerLevels = [2, 3, 4]
  } = options

  // État réactif
  const editorInstance = shallowRef<InstanceType<typeof import('@editorjs/editorjs').default> | null>(null)
  const isReady = ref(false)
  const isLoading = ref(false)
  const error = ref<string | null>(null)
  const savedData = ref<EditorJsData | null>(null)

  /**
   * Construit la configuration des outils
   */
  const buildToolsConfig = async () => {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const toolsConfig: Record<string, any> = {}

    // Import dynamique des outils pour le tree-shaking
    if (tools.includes('header')) {
      const Header = (await import('@editorjs/header')).default
      toolsConfig.header = {
        class: Header,
        config: {
          levels: headerLevels,
          defaultLevel: headerLevels[0]
        },
        inlineToolbar: true
      }
    }

    if (tools.includes('list')) {
      const List = (await import('@editorjs/list')).default
      toolsConfig.list = {
        class: List,
        inlineToolbar: true
      }
    }

    if (tools.includes('paragraph')) {
      const Paragraph = (await import('@editorjs/paragraph')).default
      toolsConfig.paragraph = {
        class: Paragraph,
        inlineToolbar: true
      }
    }

    if (tools.includes('image') && imageUploadUrl) {
      const ImageTool = (await import('@editorjs/image')).default
      toolsConfig.image = {
        class: ImageTool,
        config: {
          endpoints: {
            byFile: imageUploadUrl,
            byUrl: imageUploadUrl
          },
          field: 'image',
          types: 'image/*'
        }
      }
    }

    if (tools.includes('quote')) {
      const Quote = (await import('@editorjs/quote')).default
      toolsConfig.quote = {
        class: Quote,
        inlineToolbar: true
      }
    }

    if (tools.includes('code')) {
      const CodeTool = (await import('@editorjs/code')).default
      toolsConfig.code = {
        class: CodeTool
      }
    }

    if (tools.includes('delimiter')) {
      const Delimiter = (await import('@editorjs/delimiter')).default
      toolsConfig.delimiter = {
        class: Delimiter
      }
    }

    if (tools.includes('embed')) {
      const Embed = (await import('@editorjs/embed')).default
      toolsConfig.embed = {
        class: Embed,
        config: {
          services: {
            youtube: true,
            vimeo: true,
            twitter: true,
            instagram: true,
            facebook: true
          }
        }
      }
    }

    if (tools.includes('table')) {
      const Table = (await import('@editorjs/table')).default
      toolsConfig.table = {
        class: Table,
        inlineToolbar: true
      }
    }

    if (tools.includes('link')) {
      const LinkTool = (await import('@editorjs/link')).default
      toolsConfig.linkTool = {
        class: LinkTool
      }
    }

    if (tools.includes('marker')) {
      const Marker = (await import('@editorjs/marker')).default
      toolsConfig.marker = {
        class: Marker
      }
    }

    if (tools.includes('inlineCode')) {
      const InlineCode = (await import('@editorjs/inline-code')).default
      toolsConfig.inlineCode = {
        class: InlineCode
      }
    }

    if (tools.includes('underline')) {
      const Underline = (await import('@editorjs/underline')).default
      toolsConfig.underline = {
        class: Underline
      }
    }

    return toolsConfig
  }

  /**
   * Initialise l'éditeur
   */
  const init = async () => {
    // Côté client uniquement
    if (!import.meta.client) return

    isLoading.value = true
    error.value = null

    try {
      const EditorJS = (await import('@editorjs/editorjs')).default
      const toolsConfig = await buildToolsConfig()

      // Détruire l'instance précédente si elle existe
      if (editorInstance.value) {
        await editorInstance.value.destroy()
      }

      editorInstance.value = new EditorJS({
        holder: holderId,
        placeholder,
        data: data as OutputData,
        tools: toolsConfig,
        readOnly,
        autofocus,
        onChange: async (api: API) => {
          if (onChange) {
            const outputData = await api.saver.save()
            onChange(outputData as EditorJsData)
          }
        },
        onReady: () => {
          isReady.value = true
          isLoading.value = false
        }
      })
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Erreur lors de l\'initialisation de l\'éditeur'
      isLoading.value = false
      console.error('[useEditorJs] Erreur:', e)
    }
  }

  /**
   * Sauvegarde le contenu de l'éditeur
   */
  const save = async (): Promise<EditorJsData | null> => {
    if (!editorInstance.value || !isReady.value) {
      console.warn('[useEditorJs] Éditeur non prêt')
      return null
    }

    try {
      const data = await editorInstance.value.save()
      savedData.value = data as EditorJsData
      return savedData.value
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Erreur lors de la sauvegarde'
      console.error('[useEditorJs] Erreur sauvegarde:', e)
      return null
    }
  }

  /**
   * Efface tout le contenu
   */
  const clear = async () => {
    if (!editorInstance.value || !isReady.value) return

    try {
      await editorInstance.value.clear()
      savedData.value = null
    } catch (e) {
      console.error('[useEditorJs] Erreur clear:', e)
    }
  }

  /**
   * Charge de nouvelles données
   */
  const render = async (newData: EditorJsData) => {
    if (!editorInstance.value || !isReady.value) return

    try {
      await editorInstance.value.render(newData as OutputData)
    } catch (e) {
      console.error('[useEditorJs] Erreur render:', e)
    }
  }

  /**
   * Détruit l'instance
   */
  const destroy = async () => {
    if (editorInstance.value) {
      try {
        await editorInstance.value.destroy()
        editorInstance.value = null
        isReady.value = false
      } catch (e) {
        console.error('[useEditorJs] Erreur destroy:', e)
      }
    }
  }

  /**
   * Basculer le mode lecture seule
   */
  const toggleReadOnly = async (readOnlyState?: boolean) => {
    if (!editorInstance.value || !isReady.value) return

    try {
      await editorInstance.value.readOnly.toggle(readOnlyState)
    } catch (e) {
      console.error('[useEditorJs] Erreur toggleReadOnly:', e)
    }
  }

  // Initialisation automatique au montage
  onMounted(() => {
    init()
  })

  // Nettoyage à la destruction
  onUnmounted(() => {
    destroy()
  })

  return {
    // État
    editorInstance: readonly(editorInstance),
    isReady: readonly(isReady),
    isLoading: readonly(isLoading),
    error: readonly(error),
    savedData: readonly(savedData),

    // Méthodes
    init,
    save,
    clear,
    render,
    destroy,
    toggleReadOnly
  }
}

/**
 * Convertit les données Editor.js en HTML simple
 */
export function editorJsToHtml(data: EditorJsData): string {
  if (!data?.blocks?.length) return ''

  return data.blocks.map(block => {
    switch (block.type) {
      case 'header':
        const level = block.data.level || 2
        return `<h${level}>${block.data.text}</h${level}>`

      case 'paragraph':
        return `<p>${block.data.text}</p>`

      case 'list':
        const tag = block.data.style === 'ordered' ? 'ol' : 'ul'
        const items = (block.data.items as string[])
          .map(item => `<li>${item}</li>`)
          .join('')
        return `<${tag}>${items}</${tag}>`

      case 'quote':
        return `<blockquote><p>${block.data.text}</p><cite>${block.data.caption || ''}</cite></blockquote>`

      case 'code':
        return `<pre><code>${block.data.code}</code></pre>`

      case 'delimiter':
        return '<hr />'

      case 'image':
        const imageCaption = block.data.caption ? `<figcaption>${block.data.caption}</figcaption>` : ''
        const imageFile = block.data.file as { url?: string } | undefined
        const imageUrl = imageFile?.url || block.data.url || ''
        return `<figure><img src="${imageUrl}" alt="${block.data.caption || ''}" />${imageCaption}</figure>`

      case 'embed':
        return `<div class="embed">${block.data.embed}</div>`

      case 'table':
        const rows = (block.data.content as string[][])
          .map(row => `<tr>${row.map(cell => `<td>${cell}</td>`).join('')}</tr>`)
          .join('')
        return `<table>${rows}</table>`

      default:
        return ''
    }
  }).join('\n')
}

/**
 * Extrait le texte brut des données Editor.js
 */
export function editorJsToPlainText(data: EditorJsData): string {
  if (!data?.blocks?.length) return ''

  return data.blocks.map(block => {
    switch (block.type) {
      case 'header':
      case 'paragraph':
        return (block.data.text as string)?.replace(/<[^>]*>/g, '') || ''

      case 'list':
        return (block.data.items as string[])
          .map(item => item.replace(/<[^>]*>/g, ''))
          .join('\n')

      case 'quote':
        return (block.data.text as string)?.replace(/<[^>]*>/g, '') || ''

      case 'code':
        return block.data.code as string || ''

      default:
        return ''
    }
  }).filter(Boolean).join('\n\n')
}
