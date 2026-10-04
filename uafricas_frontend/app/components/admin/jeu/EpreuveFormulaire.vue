<script setup lang="ts">
/**
 * Formulaire de saisie d'une épreuve (back-office, daisyUI).
 *
 * La bonne réponse se désigne par un bouton radio en face de la proposition :
 * il ne peut y en avoir qu'une, c'est aussi ce que la base impose.
 */
import { LIBELLES_TYPE_REPONSE, type EpreuveForm, type ModuleAdminAPI } from '~/composables/useAdminJeu'
import type { TypeReponse } from '~/composables/useJeu'
import { PAYS_AFRICAINS_ISO2 } from '~/constants/afripulsePaysAutorises'
import { NOMS_PAYS_FR } from '~/utils/carteAfrique'

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

// ── Feature 014 : carte, ordre, paires ───────────────────────────────────
const TYPES: TypeReponse[] = ['choix', 'carte', 'ordre', 'paires']

const paysTries = PAYS_AFRICAINS_ISO2
  .map(iso => ({ iso, nom: NOMS_PAYS_FR[iso] ?? iso.toUpperCase() }))
  .sort((a, b) => a.nom.localeCompare(b.nom, 'fr'))

const changerType = (type: TypeReponse) => {
  form.value.type_reponse = type
  // Un type neuf démarre avec le minimum de lignes qu'il exige.
  if (type === 'ordre' && form.value.elements.length < 3) {
    form.value.elements = Array.from({ length: 4 }, () => ({ texte: '', valeur: null }))
  }
  if (type === 'paires' && form.value.paires.length < 3) {
    form.value.paires = Array.from({ length: 4 }, () => ({ gauche: '', droite: '' }))
  }
}

const deplacerElement = (de: number, vers: number) => {
  const liste = form.value.elements
  if (vers < 0 || vers >= liste.length) return
  const [element] = liste.splice(de, 1)
  liste.splice(vers, 0, element!)
}

const changerTypeMedia = (type: 'image' | 'audio' | null) => {
  form.value.media_type = type
  form.value.media_url = null
}

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

      <!-- Dépôt sur la route du jeu (feature 014) : EXIF retirées, son borné à
           30 s et 1 Mo. Une adresse déjà enregistrée reste affichée. -->
      <div v-if="form.media_type" class="mt-3">
        <AdminJeuChampMediaEpreuve :key="form.media_type" v-model="form.media_url" :type="form.media_type" />
      </div>
      <div v-if="form.media_type === 'audio'" class="mt-1">
        <p class="mt-1 text-xs text-base-content/60">
          Le membre peut réécouter l'extrait tant que le temps n'est pas écoulé. Une épreuve à média
          bénéficie de quelques secondes de chargement en plus.
        </p>
      </div>
    </div>

    <!-- Type de réponse (feature 014) -->
    <div>
      <span class="label-text mb-2 block font-medium">Type de réponse</span>
      <div class="join">
        <button
          v-for="t in TYPES"
          :key="t"
          type="button"
          class="btn btn-sm join-item"
          :class="form.type_reponse === t && 'btn-active'"
          @click="changerType(t)"
        >
          {{ LIBELLES_TYPE_REPONSE[t] }}
        </button>
      </div>
    </div>

    <!-- Carte -->
    <label v-if="form.type_reponse === 'carte'" class="flex flex-col">
      <span class="label-text mb-1 font-medium">Pays attendu *</span>
      <select v-model="form.reponse_pays_iso" class="select select-bordered select-sm w-full max-w-sm" required>
        <option :value="null" disabled>Choisissez le pays</option>
        <option v-for="p in paysTries" :key="p.iso" :value="p.iso">{{ p.nom }}</option>
      </select>
      <span class="mt-1 text-xs text-base-content/60">Le membre désigne ce pays sur la carte de l'Afrique, ou dans la liste.</span>
    </label>

    <!-- Ordre -->
    <div v-else-if="form.type_reponse === 'ordre'">
      <div class="mb-2 flex items-center justify-between">
        <span class="label-text font-medium">
          Éléments * <span class="font-normal text-base-content/60">(saisissez-les DANS L'ORDRE ATTENDU, l'énoncé dit le critère)</span>
        </span>
        <button
          type="button"
          class="btn btn-ghost btn-xs"
          :disabled="form.elements.length >= 6"
          @click="form.elements.push({ texte: '', valeur: null })"
        >
          <font-awesome-icon icon="plus" class="mr-1" /> Ajouter
        </button>
      </div>
      <ol class="space-y-2">
        <li v-for="(element, index) in form.elements" :key="index" class="flex items-center gap-2">
          <span class="w-6 text-right text-sm font-bold">{{ index + 1 }}.</span>
          <input v-model="element.texte" type="text" class="input input-bordered input-sm flex-1" :placeholder="`Élément ${index + 1}`">
          <input v-model="element.valeur" type="text" class="input input-bordered input-sm w-48" placeholder="Valeur (facultative)">
          <button type="button" class="btn btn-ghost btn-xs" :disabled="index === 0" :aria-label="`Monter l'élément ${index + 1}`" @click="deplacerElement(index, index - 1)">
            <font-awesome-icon icon="chevron-up" />
          </button>
          <button type="button" class="btn btn-ghost btn-xs" :disabled="index === form.elements.length - 1" :aria-label="`Descendre l'élément ${index + 1}`" @click="deplacerElement(index, index + 1)">
            <font-awesome-icon icon="chevron-down" />
          </button>
          <button type="button" class="btn btn-ghost btn-xs text-error" :disabled="form.elements.length <= 3" :aria-label="`Retirer l'élément ${index + 1}`" @click="form.elements.splice(index, 1)">
            <font-awesome-icon icon="trash" />
          </button>
        </li>
      </ol>
      <p class="mt-1 text-xs text-base-content/60">
        De trois à six éléments. La valeur (« 46 millions d'habitants », « 1960 »…) est montrée à la correction ; renseignez-la pour tous ou pour aucun.
      </p>
    </div>

    <!-- Paires -->
    <div v-else-if="form.type_reponse === 'paires'">
      <div class="mb-2 flex items-center justify-between">
        <span class="label-text font-medium">Paires *</span>
        <button
          type="button"
          class="btn btn-ghost btn-xs"
          :disabled="form.paires.length >= 5"
          @click="form.paires.push({ gauche: '', droite: '' })"
        >
          <font-awesome-icon icon="plus" class="mr-1" /> Ajouter
        </button>
      </div>
      <ul class="space-y-2">
        <li v-for="(paire, index) in form.paires" :key="index" class="flex items-center gap-2">
          <input v-model="paire.gauche" type="text" class="input input-bordered input-sm flex-1" :placeholder="`Élément ${index + 1}`">
          <font-awesome-icon icon="arrow-right" class="text-base-content/40" />
          <input v-model="paire.droite" type="text" class="input input-bordered input-sm flex-1" placeholder="Son correspondant">
          <button type="button" class="btn btn-ghost btn-xs text-error" :disabled="form.paires.length <= 3" :aria-label="`Retirer la paire ${index + 1}`" @click="form.paires.splice(index, 1)">
            <font-awesome-icon icon="trash" />
          </button>
        </li>
      </ul>
      <p class="mt-1 text-xs text-base-content/60">De trois à cinq paires. La colonne de droite est mélangée pour le membre.</p>
    </div>

    <!-- Propositions -->
    <div v-else>
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
