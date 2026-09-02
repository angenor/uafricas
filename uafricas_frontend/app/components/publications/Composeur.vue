<template>
  <section class="rounded-[10px] border border-af-bordure bg-white p-5">
    <label class="sr-only" :for="id">Partager quelque chose</label>
    <textarea
      :id="id"
      ref="champ"
      v-model="texte"
      :rows="deplie ? 4 : 2"
      placeholder="Partager quelque chose aujourd'hui…"
      class="w-full resize-none border-0 border-b border-af-bordure pb-3 text-[16px]/[1.4] placeholder:text-af-atone focus:border-af-chocolat focus:outline-none"
      @focus="deplie = true"
    />

    <!-- Les deux actions n'apparaissent qu'une fois la saisie engagée : au
         repos, le composeur est une invitation, pas un formulaire. -->
    <div v-if="deplie" class="mt-4 flex items-center justify-end gap-6">
      <button
        type="button"
        class="text-base font-bold text-af-chocolat transition hover:opacity-70"
        @click="annuler"
      >
        Annuler
      </button>
      <AfricansBouton :desactive="!texte.trim()" @click="publier">
        Publier
      </AfricansBouton>
    </div>
  </section>
</template>

<script setup lang="ts">
/**
 * Composeur du fil d'actualité.
 *
 * Il ne publie PAS lui-même : le fil agrège huit sources, aucune n'accepte un
 * texte nu. Ce qui est saisi ici part vers la modale de publication Codimoi,
 * seul module où un membre publie librement, et qui exige en plus une
 * catégorie et un territoire. Le texte suit, il n'est pas à retaper.
 */
const emit = defineEmits<{ publier: [texte: string] }>()

const id = useId()
const champ = ref<HTMLTextAreaElement | null>(null)
const texte = ref('')
const deplie = ref(false)

function annuler() {
  texte.value = ''
  deplie.value = false
  champ.value?.blur()
}

function publier() {
  const contenu = texte.value.trim()
  if (!contenu) return
  emit('publier', contenu)
  texte.value = ''
  deplie.value = false
}
</script>
