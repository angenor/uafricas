<template>
  <!-- Fil des étapes d'un formulaire long. Il n'est PAS cliquable en avant :
       on ne saute pas par-dessus une étape dont les champs obligatoires ne
       sont pas remplis. Le retour en arrière, lui, est libre — relire ce
       qu'on a saisi ne casse rien. -->
  <ol class="flex items-center gap-2">
    <li v-for="(etape, i) in etapes" :key="etape.titre" class="flex flex-1 items-center gap-2">
      <button
        type="button"
        class="flex min-w-0 flex-1 items-center gap-2 rounded-lg px-2 py-1.5 text-left transition"
        :class="[
          i === courante ? 'bg-af-chocolat/10' : '',
          i < courante ? 'hover:bg-af-fond' : '',
          i > courante ? 'cursor-default' : '',
        ]"
        :aria-current="i === courante ? 'step' : undefined"
        :disabled="i > courante"
        @click="i < courante && emit('aller', i)"
      >
        <span
          class="grid size-7 shrink-0 place-items-center rounded-full text-[12px] font-bold transition"
          :class="i < courante
            ? 'bg-af-vert text-white'
            : i === courante
              ? 'bg-af-chocolat text-white'
              : 'bg-af-fond text-af-atone-2'"
        >
          <font-awesome-icon v-if="i < courante" icon="fa-solid fa-check" />
          <template v-else>{{ i + 1 }}</template>
        </span>
        <span
          class="hidden min-w-0 truncate text-[13px]/[1.3] font-bold sm:block"
          :class="i === courante ? 'text-af-chocolat' : i < courante ? 'text-af-corps' : 'text-af-atone-2'"
        >
          {{ etape.titre }}
        </span>
      </button>

      <span
        v-if="i < etapes.length - 1"
        class="h-0.5 w-4 shrink-0 rounded-full"
        :class="i < courante ? 'bg-af-vert' : 'bg-af-bordure'"
      />
    </li>
  </ol>
</template>

<script setup lang="ts">
defineProps<{
  etapes: readonly { titre: string }[]
  courante: number
}>()

const emit = defineEmits<{ aller: [index: number] }>()
</script>
