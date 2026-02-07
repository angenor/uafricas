<script setup lang="ts">
import type { EditorJsData, EditorJsTools } from '~/composables/useEditorJs'

interface Props {
  /** ID unique pour le conteneur (requis si plusieurs éditeurs sur la page) */
  id?: string
  /** Valeur initiale (v-model) */
  modelValue?: EditorJsData
  /** Placeholder du paragraphe vide */
  placeholder?: string
  /** Outils à activer */
  tools?: EditorJsTools[]
  /** Mode lecture seule */
  readOnly?: boolean
  /** URL pour l'upload d'images */
  imageUploadUrl?: string
  /** Autofocus */
  autofocus?: boolean
  /** Niveaux de header disponibles */
  headerLevels?: number[]
  /** Hauteur minimale de l'éditeur */
  minHeight?: string
  /** Classe CSS additionnelle pour le conteneur */
  containerClass?: string
}

const props = withDefaults(defineProps<Props>(), {
  id: 'editorjs',
  placeholder: 'Commencez à écrire votre contenu...',
  tools: () => ['header', 'list', 'paragraph', 'quote', 'code', 'delimiter', 'embed', 'table', 'link', 'marker', 'inlineCode', 'underline'],
  readOnly: false,
  autofocus: false,
  headerLevels: () => [2, 3, 4],
  minHeight: '300px'
})

const emit = defineEmits<{
  'update:modelValue': [data: EditorJsData]
  'ready': []
  'error': [message: string]
}>()

const {
  isReady,
  isLoading,
  error,
  save,
  clear,
  render,
  toggleReadOnly
} = useEditorJs({
  holderId: props.id,
  placeholder: props.placeholder,
  data: props.modelValue,
  tools: props.tools,
  readOnly: props.readOnly,
  imageUploadUrl: props.imageUploadUrl,
  autofocus: props.autofocus,
  headerLevels: props.headerLevels,
  onChange: (data) => {
    emit('update:modelValue', data)
  }
})

// Émettre les événements d'état
watch(isReady, (ready) => {
  if (ready) emit('ready')
})

watch(error, (err) => {
  if (err) emit('error', err)
})

// Charger de nouvelles données quand modelValue change (si différent)
watch(() => props.modelValue, async (newData) => {
  if (isReady.value && newData) {
    await render(newData)
  }
}, { deep: true })

// Exposer les méthodes pour le parent
defineExpose({
  save,
  clear,
  render,
  toggleReadOnly,
  isReady,
  isLoading,
  error
})
</script>

<template>
  <div class="editor-js-wrapper">
    <!-- Indicateur de chargement -->
    <div
      v-if="isLoading"
      class="flex items-center justify-center py-8"
    >
      <span class="loading loading-spinner loading-lg text-primary" />
    </div>

    <!-- Message d'erreur -->
    <div
      v-if="error"
      class="alert alert-error mb-4"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        class="h-6 w-6 shrink-0 stroke-current"
        fill="none"
        viewBox="0 0 24 24"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
        />
      </svg>
      <span>{{ error }}</span>
    </div>

    <!-- Conteneur de l'éditeur -->
    <div
      :id="id"
      class="editor-js-container"
      :class="[
        containerClass,
        {
          'opacity-50 pointer-events-none': isLoading,
          'editor-js-readonly': readOnly
        }
      ]"
      :style="{ minHeight }"
    />
  </div>
</template>

<style scoped>
.editor-js-wrapper {
  @apply w-full;
}

.editor-js-container {
  @apply bg-base-100 border border-base-300 rounded-lg p-4;
  @apply prose prose-sm max-w-none;
  @apply focus-within:border-primary focus-within:ring-1 focus-within:ring-primary;
  transition: border-color 0.2s ease, box-shadow 0.2s ease;
}

.editor-js-readonly {
  @apply bg-base-200 cursor-default;
}

/* Styles pour Editor.js */
:deep(.ce-block__content) {
  max-width: 100%;
}

:deep(.ce-toolbar__content) {
  max-width: 100%;
}

:deep(.codex-editor__redactor) {
  padding-bottom: 100px !important;
}

:deep(.ce-paragraph) {
  @apply text-base-content;
}

:deep(.ce-header) {
  @apply text-base-content font-display;
}

:deep(.ce-block--selected .ce-block__content) {
  @apply bg-primary/10;
}

:deep(.cdx-quote__text) {
  @apply border-l-4 border-primary pl-4 italic;
}

:deep(.tc-table) {
  @apply border-collapse w-full;
}

:deep(.tc-table td) {
  @apply border border-base-300 p-2;
}

:deep(.ce-code__textarea) {
  @apply bg-base-200 text-base-content font-mono text-sm rounded;
}

:deep(.cdx-list__item) {
  @apply py-1;
}

:deep(.ce-delimiter::before) {
  @apply text-base-content/50;
}

/* Toolbar inline */
:deep(.ce-inline-toolbar) {
  @apply bg-base-100 border border-base-300 shadow-lg rounded-lg;
}

:deep(.ce-inline-tool) {
  @apply text-base-content hover:bg-base-200;
}

:deep(.ce-inline-tool--active) {
  @apply text-primary bg-primary/10;
}

/* Conversion toolbar */
:deep(.ce-conversion-toolbar) {
  @apply bg-base-100 border border-base-300 shadow-lg rounded-lg;
}

:deep(.ce-conversion-tool) {
  @apply text-base-content hover:bg-base-200;
}

/* Block settings */
:deep(.ce-settings) {
  @apply bg-base-100 border border-base-300 shadow-lg rounded-lg;
}

:deep(.ce-settings__button) {
  @apply text-base-content hover:bg-base-200;
}

/* Placeholder */
:deep(.ce-paragraph[data-placeholder]:empty::before) {
  @apply text-base-content/40;
}
</style>
