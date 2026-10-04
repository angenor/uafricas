<script setup lang="ts">
/**
 * Formulaire de saisie d'une épreuve (back-office, daisyUI).
 *
 * La bonne réponse se désigne par un bouton radio en face de la proposition :
 * il ne peut y en avoir qu'une, c'est aussi ce que la base impose.
 */
import type { EpreuveForm, ModuleAdminAPI } from '~/composables/useAdminJeu'

const form = defineModel<EpreuveForm>({ required: true })

defineProps<{
  modules: ModuleAdminAPI[]
  /** Une épreuve dérivée garde sa source : elle n'est pas modifiable ici. */
  desactive?: boolean
}>()

const MIN_PROPOSITIONS = 2
const MAX_PROPOSITIONS = 6

const ajouterProposition = () => {
  if (form.value.propositions.length < MAX_PROPOSITIONS) form.value.propositions.push('')
}

const retirerProposition = (index: number) => {
  if (form.value.propositions.length <= MIN_PROPOSITIONS) return
  form.value.propositions.splice(index, 1)
  // La bonne réponse est un RANG : retirer une proposition placée avant elle
  // la décalerait d'un cran sans que rien ne le signale.
  const rang = index + 1
  if (form.value.bonne_reponse === rang) form.value.bonne_reponse = 1
  else if (form.value.bonne_reponse > rang) form.value.bonne_reponse -= 1
}

const changerTypeMedia = (type: 'image' | 'audio' | null) => {
  form.value.media_type = type
  form.value.media_url = null
}

/** Les champs d'upload manipulent une chaîne ; le formulaire, `string | null`. */
const mediaUrl = computed({
  get: () => form.value.media_url ?? '',
  set: (valeur: string) => { form.value.media_url = valeur || null },
})
</script>

<template>
  <fieldset class="space-y-6" :disabled="desactive">
    <div class="grid gap-4 md:grid-cols-3">
      <label class="flex flex-col">
        <span class="label-text mb-1 font-medium">Module *</span>
        <select v-model="form.module" class="select select-bordered select-sm w-full" required>
          <option v-for="m in modules" :key="m.code" :value="m.code">{{ m.libelle }}</option>
        </select>
      </label>

      <label class="flex flex-col">
        <span class="label-text mb-1 font-medium">Difficulté</span>
        <select v-model.number="form.difficulte" class="select select-bordered select-sm w-full">
          <option :value="1">Facile</option>
          <option :value="2">Moyen</option>
          <option :value="3">Difficile</option>
        </select>
      </label>

      <label class="flex flex-col">
        <span class="label-text mb-1 font-medium">Thème</span>
        <input
          v-model="form.theme"
          type="text"
          maxlength="80"
          class="input input-bordered input-sm w-full"
          placeholder="Salutations, Écritures…"
        >
      </label>
    </div>

    <label class="flex flex-col">
      <span class="label-text mb-1 font-medium">Énoncé *</span>
      <textarea
        v-model="form.enonce"
        rows="2"
        class="textarea textarea-bordered w-full"
        placeholder="Comment dit-on « bonjour » en swahili ?"
        required
      />
    </label>

    <!-- Média de l'énoncé -->
    <div>
      <span class="label-text mb-2 block font-medium">Média de l'énoncé</span>
      <div class="join">
        <button
          type="button"
          class="btn btn-sm join-item"
          :class="form.media_type === null && 'btn-active'"
          @click="changerTypeMedia(null)"
        >
          Aucun
        </button>
        <button
          type="button"
          class="btn btn-sm join-item"
          :class="form.media_type === 'image' && 'btn-active'"
          @click="changerTypeMedia('image')"
        >
          Image
        </button>
        <button
          type="button"
          class="btn btn-sm join-item"
          :class="form.media_type === 'audio' && 'btn-active'"
          @click="changerTypeMedia('audio')"
        >
          Extrait sonore
        </button>
      </div>

      <div v-if="form.media_type === 'image'" class="mt-3">
        <OpportuniteAfriqueImageUploadField v-model="mediaUrl" label="Image de l'énoncé" />
      </div>
      <div v-else-if="form.media_type === 'audio'" class="mt-3">
        <AdminMediaUploadField v-model="mediaUrl" label="Extrait sonore" kind="audio" />
        <p class="mt-1 text-xs text-base-content/60">
          Le membre peut réécouter l'extrait tant que le temps n'est pas écoulé. Une épreuve à média
          bénéficie de quelques secondes de chargement en plus.
        </p>
      </div>
    </div>

    <!-- Propositions -->
    <div>
      <div class="mb-2 flex items-center justify-between">
        <span class="label-text font-medium">Propositions * <span class="font-normal text-base-content/60">(cochez la bonne)</span></span>
        <button
          type="button"
          class="btn btn-ghost btn-xs"
          :disabled="form.propositions.length >= MAX_PROPOSITIONS"
          @click="ajouterProposition"
        >
          <font-awesome-icon icon="plus" class="mr-1" /> Ajouter
        </button>
      </div>

      <ul class="space-y-2">
        <li
          v-for="(_, index) in form.propositions"
          :key="index"
          class="flex items-center gap-3"
        >
          <input
            v-model.number="form.bonne_reponse"
            type="radio"
            name="bonne-reponse"
            class="radio radio-success radio-sm"
            :value="index + 1"
            :aria-label="`La proposition ${index + 1} est la bonne réponse`"
          >
          <input
            v-model="form.propositions[index]"
            type="text"
            class="input input-bordered input-sm flex-1"
            :class="form.bonne_reponse === index + 1 && 'input-success'"
            :placeholder="`Proposition ${index + 1}`"
          >
          <button
            type="button"
            class="btn btn-ghost btn-xs text-error"
            :disabled="form.propositions.length <= MIN_PROPOSITIONS"
            :aria-label="`Retirer la proposition ${index + 1}`"
            @click="retirerProposition(index)"
          >
            <font-awesome-icon icon="trash" />
          </button>
        </li>
      </ul>
      <p class="mt-1 text-xs text-base-content/60">
        De deux à six propositions. L'ordre est mélangé à chaque présentation.
      </p>
    </div>

    <label class="flex flex-col">
      <span class="label-text mb-1 font-medium">Explication <span class="font-normal text-base-content/60">(obligatoire pour publier)</span></span>
      <textarea
        v-model="form.explication"
        rows="3"
        class="textarea textarea-bordered w-full"
        placeholder="Affichée au membre après sa réponse : pourquoi c'est la bonne."
      />
    </label>
  </fieldset>
</template>
