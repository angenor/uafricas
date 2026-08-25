<script setup lang="ts">
/**
 * Grille de programmation hebdomadaire d'un support média (US5, puis US2 de la
 * feature 009).
 *
 * Un créneau n'est pas un événement daté mais une règle de diffusion. La grille
 * est donc une **projection** : un créneau `quotidien` n'existe qu'une fois en
 * base, mais se lit dans les sept colonnes. Rien n'est dupliqué côté données.
 *
 * Depuis 09q le créneau désigne un **programme**, pas un contenu diffusable.
 * L'épisode qui passera se déduit de la rotation, calculée à la lecture à
 * partir de `date_effet` : déplacer cette date est le seul levier du détenteur
 * pour choisir quel épisode passe quand. Le formulaire en montre donc
 * immédiatement l'effet : sans cet aperçu, la rotation resterait une
 * abstraction que personne ne saurait piloter.
 */
import type { CreneauAPI, CreneauFormulaire, RefContenu } from '~/composables/useMediaProgrammation'
import type { TypeSupportMedia } from '~/composables/useMediaDetention'

/** Programme proposé au créneau, avec de quoi juger s'il tiendra la cadence. */
export interface EmissionProgrammable {
  id: string
  titre: string
  cadence?: string
  nombre_episodes?: number
}

const props = withDefaults(defineProps<{
  typeSupport: TypeSupportMedia
  supportId: string
  /** Programmes du support : la seule source du sélecteur. */
  emissions: EmissionProgrammable[]
  modifiable?: boolean
}>(), { modifiable: false })

const emit = defineEmits<{ maj: [] }>()

const {
  chargement,
  erreur,
  listerGrille,
  creerCreneau,
  modifierCreneau,
  supprimerCreneau,
} = useMediaProgrammation()

const creneaux = ref<CreneauAPI[]>([])

/** Le détenteur voit aussi les créneaux qui n'annoncent rien : ce sont
 * précisément ceux qu'il doit corriger (FR-021). */
const charger = async () => {
  creneaux.value = await listerGrille(props.typeSupport, props.supportId, props.modifiable)
}

onMounted(charger)

/**
 * Fuseau de référence de la grille : celui du plus grand nombre de créneaux.
 * Les autres sont annotés créneau par créneau (FR-026), annoter les vingt
 * lignes d'une grille homogène n'apprendrait rien à personne.
 */
const fuseauMajoritaire = computed(() => {
  const comptes = new Map<string, number>()
  for (const c of creneaux.value) comptes.set(c.fuseau, (comptes.get(c.fuseau) ?? 0) + 1)
  let gagnant = FUSEAU_DEFAUT
  let meilleur = -1
  for (const [fuseau, n] of comptes) {
    if (n > meilleur) { gagnant = fuseau; meilleur = n }
  }
  return gagnant
})

/** Sept colonnes indexées comme `EXTRACT(DOW)` : 0 = dimanche. */
const colonnes = computed(() =>
  JOURS_SEMAINE.map((libelle, jour) => ({
    jour,
    libelle,
    creneaux: creneaux.value
      .filter(c => c.recurrence === 'quotidien' || c.jour_semaine === jour)
      .sort((a, b) => a.heure_debut.localeCompare(b.heure_debut)),
  })),
)

const grilleVide = computed(() => creneaux.value.length === 0)

const emissionDe = (creneau: CreneauAPI): EmissionProgrammable | undefined =>
  props.emissions.find(e => e.id === creneau.emission_id)

const nomEmission = (creneau: CreneauAPI) =>
  creneau.emission?.titre ?? emissionDe(creneau)?.titre ?? 'Programme inconnu'

const LIBELLES_CADENCE_COURTE: Record<string, string> = {
  quotidienne: 'Quotidien',
  hebdomadaire: 'Hebdo',
  ponctuelle: 'Ponctuel',
}

// ---------------------------------------------------------------------------
// Formulaire (création et édition partagent le même panneau)
// ---------------------------------------------------------------------------

const panneauOuvert = ref(false)
const creneauEdite = ref<CreneauAPI | null>(null)
const enregistrement = ref(false)
/** Épisode que le serveur a retenu au dernier enregistrement, l'aperçu. */
const apercuEpisode = ref<RefContenu | null>(null)

const formulaire = reactive<CreneauFormulaire>({
  emission_id: '',
  recurrence: 'hebdomadaire',
  jour_semaine: 1,
  heure_debut: '20:00',
  duree_minutes: 60,
  fuseau: FUSEAU_DEFAUT,
  date_effet: dateAujourdhui(),
})

const emissionChoisie = computed(() =>
  props.emissions.find(e => e.id === formulaire.emission_id),
)

const reinitialiser = () => {
  formulaire.emission_id = props.emissions[0]?.id ?? ''
  formulaire.recurrence = 'hebdomadaire'
  formulaire.jour_semaine = 1
  formulaire.heure_debut = '20:00'
  formulaire.duree_minutes = 60
  formulaire.fuseau = fuseauMajoritaire.value
  formulaire.date_effet = dateAujourdhui()
  apercuEpisode.value = null
}

const ouvrirCreation = (jour?: number) => {
  creneauEdite.value = null
  reinitialiser()
  if (jour !== undefined) formulaire.jour_semaine = jour
  panneauOuvert.value = true
}

const ouvrirEdition = (creneau: CreneauAPI) => {
  if (!props.modifiable) return
  creneauEdite.value = creneau
  formulaire.emission_id = creneau.emission_id
  formulaire.recurrence = creneau.recurrence
  formulaire.jour_semaine = creneau.jour_semaine ?? 1
  formulaire.heure_debut = creneau.heure_debut.slice(0, 5)
  formulaire.duree_minutes = creneau.duree_minutes
  formulaire.fuseau = creneau.fuseau
  formulaire.date_effet = creneau.date_effet
  apercuEpisode.value = creneau.episode ?? null
  panneauOuvert.value = true
}

const fermerPanneau = () => {
  panneauOuvert.value = false
  creneauEdite.value = null
  apercuEpisode.value = null
}

const soumettre = async () => {
  if (!formulaire.emission_id) return
  enregistrement.value = true
  const resultat = creneauEdite.value
    ? await modifierCreneau(creneauEdite.value.id, { ...formulaire })
    : await creerCreneau(props.typeSupport, props.supportId, { ...formulaire })
  enregistrement.value = false
  // Un chevauchement (409) laisse le panneau ouvert : le message du serveur
  // désigne le créneau en cause, l'utilisateur doit pouvoir corriger sa saisie.
  if (!resultat) return
  // L'épisode retenu revient du serveur : c'est ce qui rend la date d'effet
  // lisible. Le panneau reste ouvert le temps de le montrer.
  apercuEpisode.value = resultat.episode_actuel ?? null
  await charger()
  emit('maj')
  if (!apercuEpisode.value) fermerPanneau()
  else creneauEdite.value = creneaux.value.find(c => c.id === resultat.id) ?? creneauEdite.value
}

const supprimer = async (creneau: CreneauAPI) => {
  if (!confirm(`Supprimer le créneau de ${creneau.heure_debut.slice(0, 5)} ?`)) return
  if (!await supprimerCreneau(creneau.id)) return
  fermerPanneau()
  await charger()
  emit('maj')
}

const dateLisible = (iso: string) => {
  const [a, m, j] = iso.split('-')
  return `${j}/${m}/${a}`
}
</script>

<template>
  <section class="text-white">
    <header class="flex flex-wrap items-center justify-between gap-4 mb-5">
      <div>
        <h2 class="text-xl sm:text-2xl font-bold">Grille de programmation</h2>
        <p class="text-sm text-gray-400">
          Horaires indiqués en
          <span class="text-yellow-400">{{ fuseauMajoritaire }}</span>
          sauf mention contraire.
        </p>
      </div>
      <button
        v-if="modifiable"
        type="button"
        class="inline-flex items-center gap-2 rounded-full border border-yellow-400 bg-yellow-400/10 text-yellow-400 px-5 py-2 text-sm hover:bg-yellow-400/20 transition-colors"
        @click="ouvrirCreation()"
      >
        <font-awesome-icon :icon="['fas', 'plus']" />
        Ajouter un créneau
      </button>
    </header>

    <!-- Message du serveur affiché tel quel : sur un chevauchement (409), c'est
         lui qui nomme le créneau en conflit. Le reformuler perdrait l'info. -->
    <p
      v-if="erreur"
      class="mb-5 rounded-lg border border-red-500 bg-red-500/10 text-red-300 px-4 py-3 text-sm"
    >
      <font-awesome-icon :icon="['fas', 'triangle-exclamation']" class="mr-2" />
      {{ erreur }}
    </p>

    <div v-if="chargement && !creneaux.length" class="flex justify-center py-12">
      <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-yellow-400" />
    </div>

    <div
      v-else-if="grilleVide"
      class="rounded-xl border border-white/10 bg-neutral-900 px-6 py-12 text-center"
    >
      <font-awesome-icon :icon="['fas', 'calendar']" class="text-3xl text-neutral-600 mb-3" />
      <p class="text-gray-400">Aucun créneau programmé.</p>
      <button
        v-if="modifiable"
        type="button"
        class="mt-4 text-sm text-yellow-400 hover:underline"
        @click="ouvrirCreation()"
      >
        Créer le premier créneau
      </button>
    </div>

    <!-- Sept colonnes au-delà de md ; en dessous, sept blocs empilés : une
         grille à sept colonnes sur mobile ne se lit tout simplement pas. -->
    <div v-else class="grid grid-cols-1 md:grid-cols-7 gap-3">
      <div
        v-for="colonne in colonnes"
        :key="colonne.jour"
        class="rounded-xl border border-white/10 bg-neutral-900 p-3 min-w-0"
      >
        <div class="flex items-center justify-between mb-3">
          <h3 class="text-sm font-bold uppercase tracking-wide text-gray-300">
            {{ colonne.libelle }}
          </h3>
          <button
            v-if="modifiable"
            type="button"
            class="text-gray-500 hover:text-yellow-400 transition-colors"
            :title="`Ajouter un créneau le ${colonne.libelle.toLowerCase()}`"
            @click="ouvrirCreation(colonne.jour)"
          >
            <font-awesome-icon :icon="['fas', 'plus']" />
          </button>
        </div>

        <p v-if="!colonne.creneaux.length" class="text-xs text-gray-600 py-2">
          Aucun créneau.
        </p>

        <ul v-else class="space-y-2">
          <li
            v-for="creneau in colonne.creneaux"
            :key="`${colonne.jour}-${creneau.id}`"
            class="rounded-lg border px-3 py-2 transition-colors"
            :class="[
              creneau.emission_indisponible
                ? 'border-red-500 bg-red-500/5'
                : 'border-white/10 bg-neutral-800 hover:border-yellow-400/60',
              modifiable ? 'cursor-pointer' : '',
            ]"
            @click="ouvrirEdition(creneau)"
          >
            <p
              class="text-sm font-semibold"
              :class="creneau.emission_indisponible ? 'text-red-300' : 'text-yellow-400'"
            >
              {{ creneau.heure_debut.slice(0, 5) }} →
              {{ heureFin(creneau.heure_debut.slice(0, 5), creneau.duree_minutes) }}
            </p>
            <p
              class="text-sm truncate"
              :class="creneau.emission_indisponible ? 'text-red-200' : 'text-white'"
              :title="nomEmission(creneau)"
            >
              {{ nomEmission(creneau) }}
            </p>
            <!-- L'épisode n'est résolu que sur les endpoints de diffusion : la
                 grille annonce la série, pas le numéro du jour. -->
            <p v-if="creneau.episode" class="text-xs text-gray-400 truncate">
              {{ creneau.episode.titre }}
            </p>

            <div class="flex flex-wrap items-center gap-1.5 mt-1.5">
              <span
                v-if="creneau.recurrence === 'quotidien'"
                class="rounded-full bg-white/10 text-gray-300 text-[10px] px-2 py-0.5 uppercase tracking-wide"
              >
                Quotidien
              </span>
              <span
                v-if="emissionDe(creneau)?.nombre_episodes !== undefined"
                class="rounded-full bg-white/10 text-gray-300 text-[10px] px-2 py-0.5"
              >
                {{ emissionDe(creneau)!.nombre_episodes }} ép.
              </span>
              <span
                v-if="emissionDe(creneau)?.cadence"
                class="rounded-full bg-white/10 text-gray-400 text-[10px] px-2 py-0.5"
              >
                {{ LIBELLES_CADENCE_COURTE[emissionDe(creneau)!.cadence!] || emissionDe(creneau)!.cadence }}
              </span>
              <!-- Le fuseau n'apparaît que s'il s'écarte de celui de l'en-tête. -->
              <span
                v-if="creneau.fuseau !== fuseauMajoritaire"
                class="rounded-full border border-yellow-400/40 text-yellow-400 text-[10px] px-2 py-0.5"
              >
                {{ creneau.fuseau }}
              </span>
            </div>

            <p v-if="creneau.emission_indisponible" class="mt-1.5 text-[11px] text-red-400 leading-snug">
              <font-awesome-icon :icon="['fas', 'triangle-exclamation']" class="mr-1" />
              {{ creneau.alerte || 'Programme indisponible : ce créneau n’annonce rien.' }}
            </p>
          </li>
        </ul>
      </div>
    </div>

    <!-- Modale maison (Tailwind pur) : création et édition d'un créneau. -->
    <div
      v-if="panneauOuvert"
      class="fixed inset-0 z-50 flex items-end sm:items-center justify-center bg-black/70 p-0 sm:p-4"
      @click.self="fermerPanneau"
    >
      <div class="w-full sm:max-w-lg max-h-[90vh] overflow-y-auto rounded-t-2xl sm:rounded-2xl border border-white/10 bg-neutral-900 p-6">
        <div class="flex items-start justify-between gap-4 mb-5">
          <h3 class="text-lg font-bold text-white">
            {{ creneauEdite ? 'Modifier le créneau' : 'Nouveau créneau' }}
          </h3>
          <button
            type="button"
            class="text-gray-500 hover:text-white transition-colors"
            aria-label="Fermer"
            @click="fermerPanneau"
          >
            <font-awesome-icon :icon="['fas', 'xmark']" />
          </button>
        </div>

        <p
          v-if="erreur"
          class="mb-4 rounded-lg border border-red-500 bg-red-500/10 text-red-300 px-4 py-3 text-sm"
        >
          {{ erreur }}
        </p>

        <form class="space-y-4" @submit.prevent="soumettre">
          <div>
            <label class="block text-sm text-gray-400 mb-1.5" for="creneau-emission">Programme</label>
            <select
              id="creneau-emission"
              v-model="formulaire.emission_id"
              required
              class="w-full rounded-lg bg-neutral-800 border border-white/10 text-white px-3 py-2 text-sm focus:border-yellow-400 focus:outline-none"
            >
              <option value="" disabled>Choisir un programme…</option>
              <option v-for="emission in emissions" :key="emission.id" :value="emission.id">
                {{ emission.titre }}
              </option>
            </select>
            <p v-if="emissionChoisie" class="text-xs text-gray-500 mt-1.5">
              {{ emissionChoisie.nombre_episodes ?? 0 }} épisode(s) publié(s)
              <template v-if="emissionChoisie.cadence">
                · cadence {{ LIBELLES_CADENCE_COURTE[emissionChoisie.cadence] || emissionChoisie.cadence }}
              </template>
            </p>
            <p v-if="emissionChoisie && !emissionChoisie.nombre_episodes" class="text-xs text-amber-400 mt-1">
              Sans épisode publié, ce créneau n’annoncera rien au public.
            </p>
          </div>

          <div class="grid sm:grid-cols-2 gap-4">
            <div>
              <label class="block text-sm text-gray-400 mb-1.5" for="creneau-recurrence">Récurrence</label>
              <select
                id="creneau-recurrence"
                v-model="formulaire.recurrence"
                class="w-full rounded-lg bg-neutral-800 border border-white/10 text-white px-3 py-2 text-sm focus:border-yellow-400 focus:outline-none"
              >
                <option value="hebdomadaire">Chaque semaine</option>
                <option value="quotidien">Tous les jours</option>
              </select>
            </div>

            <!-- Un créneau quotidien ne porte pas de jour : le champ disparaît
                 et la valeur part à `null` (CHECK ck_creneau_jour_coherent). -->
            <div v-if="formulaire.recurrence === 'hebdomadaire'">
              <label class="block text-sm text-gray-400 mb-1.5" for="creneau-jour">Jour</label>
              <select
                id="creneau-jour"
                v-model.number="formulaire.jour_semaine"
                class="w-full rounded-lg bg-neutral-800 border border-white/10 text-white px-3 py-2 text-sm focus:border-yellow-400 focus:outline-none"
              >
                <option v-for="(libelle, index) in JOURS_SEMAINE" :key="libelle" :value="index">
                  {{ libelle }}
                </option>
              </select>
            </div>
          </div>

          <div class="grid sm:grid-cols-2 gap-4">
            <div>
              <label class="block text-sm text-gray-400 mb-1.5" for="creneau-heure">Heure de début</label>
              <input
                id="creneau-heure"
                v-model="formulaire.heure_debut"
                type="time"
                required
                class="w-full rounded-lg bg-neutral-800 border border-white/10 text-white px-3 py-2 text-sm focus:border-yellow-400 focus:outline-none"
              >
            </div>
            <div>
              <label class="block text-sm text-gray-400 mb-1.5" for="creneau-duree">Durée (minutes)</label>
              <input
                id="creneau-duree"
                v-model.number="formulaire.duree_minutes"
                type="number"
                min="1"
                max="1440"
                required
                class="w-full rounded-lg bg-neutral-800 border border-white/10 text-white px-3 py-2 text-sm focus:border-yellow-400 focus:outline-none"
              >
            </div>
          </div>

          <div>
            <label class="block text-sm text-gray-400 mb-1.5" for="creneau-fuseau">Fuseau horaire</label>
            <select
              id="creneau-fuseau"
              v-model="formulaire.fuseau"
              class="w-full rounded-lg bg-neutral-800 border border-white/10 text-white px-3 py-2 text-sm focus:border-yellow-400 focus:outline-none"
            >
              <option v-for="fuseau in FUSEAUX_PROPOSES" :key="fuseau" :value="fuseau">
                {{ fuseau }}
              </option>
            </select>
          </div>

          <!-- Date d'effet : l'origine du comptage des occurrences. C'est elle
               qui décide quel épisode passe quand. -->
          <div>
            <label class="block text-sm text-gray-400 mb-1.5" for="creneau-date-effet">
              Date d’effet de la rotation
            </label>
            <input
              id="creneau-date-effet"
              v-model="formulaire.date_effet"
              type="date"
              class="w-full rounded-lg bg-neutral-800 border border-white/10 text-white px-3 py-2 text-sm focus:border-yellow-400 focus:outline-none"
            >
            <p class="text-xs text-gray-500 mt-1.5">
              Le premier épisode du programme passe à cette date ; les suivants suivent
              l’ordre des épisodes, puis la série recommence.
            </p>
          </div>

          <p class="text-xs text-gray-500">
            Fin prévue à
            <span class="text-gray-300">
              {{ heureFin(formulaire.heure_debut, formulaire.duree_minutes) }}
            </span>.
          </p>

          <!-- Aperçu renvoyé par le serveur après enregistrement : la rotation
               cesse d'être une abstraction. -->
          <div
            v-if="apercuEpisode"
            class="rounded-lg border border-yellow-400/40 bg-yellow-400/5 px-4 py-3"
          >
            <p class="text-xs uppercase tracking-wide text-yellow-400 mb-1">
              Prochaine occurrence
            </p>
            <p class="text-sm text-white">
              <span v-if="apercuEpisode.numero_episode" class="text-gray-400">
                Épisode {{ apercuEpisode.numero_episode }}, 
              </span>
              {{ apercuEpisode.titre }}
            </p>
            <p v-if="creneauEdite" class="text-xs text-gray-400 mt-1">
              Rotation calée sur le {{ dateLisible(creneauEdite.date_effet) }}
              <template v-if="creneauEdite.est_rediffusion"> · rediffusion</template>
            </p>
          </div>

          <div class="flex flex-wrap items-center justify-between gap-3 pt-2">
            <button
              v-if="creneauEdite"
              type="button"
              class="inline-flex items-center gap-2 rounded-full border border-red-500 text-red-400 px-4 py-2 text-sm hover:bg-red-500/10 transition-colors"
              @click="supprimer(creneauEdite)"
            >
              <font-awesome-icon :icon="['fas', 'trash']" />
              Supprimer
            </button>
            <span v-else />

            <div class="flex items-center gap-3">
              <button
                type="button"
                class="rounded-full border border-white/20 text-gray-300 px-4 py-2 text-sm hover:bg-white/5 transition-colors"
                @click="fermerPanneau"
              >
                {{ apercuEpisode ? 'Fermer' : 'Annuler' }}
              </button>
              <button
                type="submit"
                :disabled="enregistrement || !formulaire.emission_id"
                class="inline-flex items-center gap-2 rounded-full bg-yellow-400 text-neutral-900 font-semibold px-5 py-2 text-sm hover:bg-yellow-300 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
              >
                <span
                  v-if="enregistrement"
                  class="animate-spin rounded-full h-4 w-4 border-b-2 border-neutral-900"
                />
                {{ creneauEdite ? 'Enregistrer' : 'Ajouter' }}
              </button>
            </div>
          </div>
        </form>
      </div>
    </div>
  </section>
</template>
