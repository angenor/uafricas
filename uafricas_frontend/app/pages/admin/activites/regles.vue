<script setup lang="ts">
/**
 * Règles du jeu (feature 013) : les valeurs réglables sans nouvelle livraison.
 *
 * Un changement ne vaut que pour l'AVENIR : les séries déjà composées (parties,
 * défis, duels) sont figées, et les gains déjà écrits ne sont pas recalculés.
 * Les montants de RÉPUTATION ne sont pas ici : ils sont dans le barème
 * d'engagement, avec les autres règles.
 */
import type { ApiResponse } from '~/types/admin'
import { messageErreurAdminJeu } from '~/composables/useAdminJeu'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

type Regles = Record<string, number>

const { adminFetch } = useAdmin()

const regles = ref<Regles | null>(null)
const chargement = ref(true)
const enCours = ref(false)
const erreur = ref('')
const message = ref('')

/** Les champs, groupés, avec leurs bornes (celles des CHECK de `jeu.regles`). */
const GROUPES: Array<{ titre: string, champs: Array<{ cle: string, libelle: string, min: number, max?: number, unite?: string }> }> = [
  { titre: 'Séries', champs: [
    { cle: 'taille_partie', libelle: 'Épreuves par partie', min: 3, max: 30 },
    { cle: 'taille_defi_jour', libelle: 'Épreuves du défi du jour', min: 3, max: 30 },
    { cle: 'taille_defi_semaine', libelle: 'Épreuves du défi de la semaine', min: 3, max: 30 },
    { cle: 'taille_duel', libelle: 'Épreuves par duel', min: 3, max: 30 },
    { cle: 'temps_epreuve_s', libelle: 'Temps par épreuve', min: 5, max: 120, unite: 's' },
  ] },
  { titre: 'Score', champs: [
    { cle: 'score_facile', libelle: 'Épreuve facile', min: 1 },
    { cle: 'score_moyen', libelle: 'Épreuve moyenne', min: 1 },
    { cle: 'score_difficile', libelle: 'Épreuve difficile', min: 1 },
    { cle: 'prime_defi_jour', libelle: 'Prime du défi du jour', min: 0 },
    { cle: 'prime_defi_semaine', libelle: 'Prime du défi de la semaine', min: 0 },
    { cle: 'prime_duel_victoire', libelle: 'Prime de victoire en duel', min: 0 },
    { cle: 'prime_duel_nul', libelle: 'Prime de duel nul (chacun)', min: 0 },
  ] },
  { titre: 'Duels', champs: [
    { cle: 'delai_duel_h', libelle: 'Délai d\'un duel différé', min: 1, max: 168, unite: 'h' },
    { cle: 'delai_direct_min', libelle: 'Délai pour accepter un duel direct', min: 1, max: 60, unite: 'min' },
    { cle: 'grace_direct_s', libelle: 'Absence tolérée en direct', min: 10, max: 300, unite: 's' },
    { cle: 'pause_revelation_s', libelle: 'Pause de correction en direct', min: 2, max: 15, unite: 's' },
    { cle: 'duels_comptes_par_paire_jour', libelle: 'Duels comptés par paire et par jour', min: 0 },
    { cle: 'duels_comptes_par_membre_jour', libelle: 'Duels comptés par membre et par jour', min: 0 },
  ] },
  { titre: 'Championship', champs: [
    { cle: 'joueurs_par_pays', libelle: 'Meilleurs joueurs comptés par pays', min: 1, max: 100 },
  ] },
]

const charger = async () => {
  try {
    regles.value = (await adminFetch<ApiResponse<Regles>>('/api/admin/jeu/regles')).data
  }
  catch (e) {
    erreur.value = messageErreurAdminJeu(e, 'Impossible de charger les règles.')
  }
  finally {
    chargement.value = false
  }
}

const enregistrer = async () => {
  if (!regles.value) return
  erreur.value = ''
  enCours.value = true
  try {
    regles.value = (await adminFetch<ApiResponse<Regles>>('/api/admin/jeu/regles', {
      method: 'PUT', body: regles.value,
    })).data
    message.value = 'Règles enregistrées. Elles valent pour les prochaines parties, défis et duels.'
    setTimeout(() => { message.value = '' }, 5000)
  }
  catch (e) {
    erreur.value = messageErreurAdminJeu(e)
  }
  finally {
    enCours.value = false
  }
}

onMounted(charger)
</script>

<template>
  <div>
    <AdminPageHeader titre="Règles du jeu" sous-titre="Les valeurs des activités, réglables sans nouvelle livraison">
      <template #actions>
        <NuxtLink to="/admin/engagement/regles" class="btn btn-ghost btn-sm">
          Montants de réputation <font-awesome-icon icon="arrow-right" class="ml-1" />
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div v-if="message" class="alert alert-success mb-4 text-sm">{{ message }}</div>
    <div v-if="erreur" class="alert alert-error mb-4 text-sm" role="alert">{{ erreur }}</div>

    <div class="alert mb-4 text-sm">
      <font-awesome-icon icon="circle-info" />
      <span>
        Un changement ne vaut que pour l'avenir : les parties, défis et duels déjà composés gardent leur série,
        et les scores déjà gagnés ne sont pas recalculés.
      </span>
    </div>

    <div v-if="chargement" class="flex justify-center py-16">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <form v-else-if="regles" class="space-y-4" @submit.prevent="enregistrer">
      <section v-for="groupe in GROUPES" :key="groupe.titre" class="card bg-base-100 shadow-sm">
        <div class="card-body gap-4">
          <h2 class="text-lg font-bold">{{ groupe.titre }}</h2>
          <div class="grid gap-4 md:grid-cols-2 xl:grid-cols-3">
            <label v-for="champ in groupe.champs" :key="champ.cle" class="flex flex-col">
              <span class="label-text mb-1 font-medium">
                {{ champ.libelle }}<template v-if="champ.unite"> ({{ champ.unite }})</template>
              </span>
              <input
                v-model.number="regles[champ.cle]"
                type="number"
                class="input input-bordered input-sm w-full"
                :min="champ.min"
                :max="champ.max"
                required
              >
            </label>
          </div>
        </div>
      </section>

      <div class="flex justify-end">
        <button type="submit" class="btn btn-primary btn-sm" :disabled="enCours">Enregistrer les règles</button>
      </div>
    </form>
  </div>
</template>
