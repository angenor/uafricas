<script setup lang="ts">
/**
 * Gestion des **épisodes** d'un programme (feature 009, US1 — FR-014 à FR-020,
 * FR-042).
 *
 * Un seul composant pour le co-détenteur et pour l'administration : les deux
 * font exactement la même chose sur les mêmes objets, et seule l'autorité
 * diffère. Ce qui change est encapsulé ici :
 *   — membre : l'épisode naît `en_attente`, le serveur en décide seul ;
 *   — admin  : il naît `publie`, l'administration étant l'autorité de validation.
 * Deux composants jumeaux auraient dupliqué le réordonnancement, la mise à la
 * une et le formulaire pour n'en changer que la route.
 *
 * Tailwind v4 pur : il est monté sur `/mon-compte/mes-supports`, page publique
 * où daisyUI est proscrit (principe VI).
 */
import type { EpisodeAPI } from '~/composables/useMediaEmissions'
import { LIBELLES_ETAT_EPISODE } from '~/composables/useMediaEmissions'

const props = withDefaults(defineProps<{
  emissionId: string
  /** Titre affiché en tête — évite un aller-retour pour le relire. */
  emissionTitre?: string
  /** `chaine_tv` → vidéo, `station_radio` → audio : décide du libellé du média. */
  typeSupport?: 'chaine_tv' | 'station_radio'
  /** Passe par les routes `/api/admin/medias/…` et publie directement. */
  admin?: boolean
  sombre?: boolean
}>(), {
  emissionTitre: '',
  typeSupport: 'chaine_tv',
  admin: false,
  sombre: true,
})

const emit = defineEmits<{ change: [] }>()

const membre = useMediaEmissions()
const administration = useAdminMediaEmissions()

const episodes = ref<EpisodeAPI[]>([])
const chargement = ref(false)
const erreur = ref<string | null>(null)
const succes = ref<string | null>(null)

const libelleMedia = computed(() =>
  props.typeSupport === 'station_radio' ? 'Fichier ou lien audio' : 'Fichier ou lien vidéo',
)

const annoncer = (message: string) => {
  succes.value = message
  setTimeout(() => { succes.value = null }, 3000)
}

const charger = async () => {
  chargement.value = true
  erreur.value = null
  try {
    episodes.value = props.admin
      ? (await administration.chargerEpisodes(props.emissionId)) as unknown as EpisodeAPI[]
      : await membre.listerEpisodesDetenteur(props.emissionId)
  }
  catch (e: any) {
    erreur.value = e?.data?.error || e?.message || 'Erreur de chargement'
  }
  finally {
    chargement.value = false
  }
}

watch(() => props.emissionId, charger)
onMounted(charger)

// ── Formulaire d'ajout / de modification ──────────────────────
const enEdition = ref<EpisodeAPI | null>(null)
const showFormulaire = ref(false)
const formulaire = reactive({
  titre: '',
  description: '',
  media_url: '',
  image_couverture_url: '',
  numero_episode: null as number | null,
  duree_minutes: null as number | null,
})

const ouvrirAjout = () => {
  enEdition.value = null
  formulaire.titre = ''
  formulaire.description = ''
  formulaire.media_url = ''
  formulaire.image_couverture_url = ''
  // Numérotation proposée, jamais imposée : une série peut repartir à 1 par saison.
  formulaire.numero_episode = episodes.value.length
    ? Math.max(...episodes.value.map(e => e.numero_episode ?? 0)) + 1
    : 1
  formulaire.duree_minutes = null
  erreur.value = null
  showFormulaire.value = true
}

const ouvrirEdition = (episode: EpisodeAPI) => {
  enEdition.value = episode
  formulaire.titre = episode.titre
  formulaire.description = episode.description || ''
  formulaire.media_url = episode.media_url || ''
  formulaire.image_couverture_url = episode.image_couverture_url || ''
  formulaire.numero_episode = episode.numero_episode
  formulaire.duree_minutes = episode.duree_minutes
  erreur.value = null
  showFormulaire.value = true
}

const enregistrer = async () => {
  erreur.value = null
  if (!formulaire.titre.trim()) { erreur.value = "Le titre de l'épisode est obligatoire."; return }
  if (!enEdition.value && !formulaire.media_url.trim()) {
    erreur.value = 'Un épisode ne peut pas être créé sans son média.'
    return
  }

  const corps: Record<string, unknown> = {
    titre: formulaire.titre.trim(),
    description: formulaire.description.trim(),
    media_url: formulaire.media_url.trim() || null,
    image_couverture_url: formulaire.image_couverture_url.trim() || null,
    numero_episode: formulaire.numero_episode,
    duree_minutes: formulaire.duree_minutes,
  }

  try {
    if (enEdition.value) {
      const ok = props.admin
        ? !!(await administration.modifierEpisode(enEdition.value.id, corps as any))
        : await membre.modifierEpisode(enEdition.value.id, corps)
      if (!ok && !props.admin) { erreur.value = membre.erreur.value; return }
      annoncer('Épisode mis à jour.')
    }
    else {
      const cree = props.admin
        ? await administration.creerEpisode(props.emissionId, corps as any)
        : await membre.creerEpisode(props.emissionId, corps)
      if (!cree) { erreur.value = membre.erreur.value || 'Création impossible'; return }
      annoncer(props.admin
        ? 'Épisode publié.'
        : 'Épisode soumis : il sera visible du public après validation.')
    }
    showFormulaire.value = false
    await charger()
    emit('change')
  }
  catch (e: any) {
    erreur.value = e?.data?.error || e?.message || 'Erreur'
  }
}

// ── Suppression ───────────────────────────────────────────────
const cibleSuppression = ref<EpisodeAPI | null>(null)

const executerSuppression = async () => {
  if (!cibleSuppression.value) return
  try {
    if (props.admin) await administration.supprimerEpisode(cibleSuppression.value.id)
    else await membre.supprimerEpisode(cibleSuppression.value.id)
    cibleSuppression.value = null
    annoncer('Épisode supprimé.')
    await charger()
    emit('change')
  }
  catch (e: any) {
    erreur.value = e?.data?.error || e?.message || 'Suppression impossible'
    cibleSuppression.value = null
  }
}

// ── Réordonnancement ──────────────────────────────────────────
/**
 * L'ordre décide de la **rotation** en grille : deux épisodes au même rang la
 * rendraient non déterministe. Le serveur exige donc une liste couvrant
 * exactement les épisodes du programme — on la renvoie toujours entière, jamais
 * la seule paire déplacée.
 */
const deplacer = async (index: number, delta: number) => {
  const cible = index + delta
  if (cible < 0 || cible >= episodes.value.length) return
  const copie = [...episodes.value]
  const [retire] = copie.splice(index, 1)
  copie.splice(cible, 0, retire!)
  episodes.value = copie

  const ordres = copie.map((e, i) => ({ episode_id: e.id, ordre: i }))
  try {
    if (props.admin) await administration.reordonnerEpisodes(props.emissionId, ordres)
    else await membre.reordonnerEpisodes(props.emissionId, ordres)
    await charger()
    emit('change')
  }
  catch (e: any) {
    erreur.value = e?.data?.error || e?.message || 'Réordonnancement refusé'
    await charger()
  }
}

// ── Mise à la une (un seul épisode par programme) ─────────────
const basculerALaUne = async (episode: EpisodeAPI) => {
  try {
    if (props.admin) await administration.definirALaUne(episode.id)
    else await membre.mettreALaUne(episode.id)
    await charger()
    emit('change')
  }
  catch (e: any) {
    erreur.value = e?.data?.error || e?.message || 'Opération refusée'
  }
}

const badgeEtat = (etat: string) => {
  if (etat === 'publie') return 'bg-green-500/15 text-green-400 border-green-500/30'
  if (etat === 'en_attente') return 'bg-amber-500/15 text-amber-400 border-amber-500/30'
  if (etat === 'rejete' || etat === 'suspendu') return 'bg-red-500/15 text-red-400 border-red-500/30'
  return 'bg-white/5 text-gray-400 border-white/15'
}

const champClasses = computed(() => props.sombre
  ? 'w-full rounded-lg px-3 py-2 text-sm bg-white/5 border border-white/15 text-white placeholder-gray-500 outline-none focus:border-yellow-400'
  : 'w-full rounded-lg px-3 py-2 text-sm bg-white border border-gray-300 text-gray-900 placeholder-gray-400 outline-none focus:border-gray-900')
</script>

<template>
  <section>
    <div class="flex flex-wrap items-baseline justify-between gap-3 mb-4">
      <h3 class="font-oswald text-lg font-bold" :class="sombre ? 'text-white' : 'text-gray-900'">
        Épisodes
        <span v-if="emissionTitre" :class="sombre ? 'text-gray-500 font-normal' : 'text-gray-400 font-normal'">
          — {{ emissionTitre }}
        </span>
      </h3>
      <button
        type="button"
        class="rounded-full px-4 py-1.5 text-sm font-semibold transition-colors"
        :class="sombre
          ? 'bg-yellow-400 text-neutral-900 hover:bg-yellow-300'
          : 'bg-gray-900 text-white hover:bg-gray-700'"
        @click="ouvrirAjout"
      >
        + Ajouter un épisode
      </button>
    </div>

    <p
      v-if="!admin"
      class="text-xs mb-4"
      :class="sombre ? 'text-gray-500' : 'text-gray-500'"
    >
      Tout épisode que vous versez part en validation. Un refus vous parvient avec son motif,
      et l'épisode reste modifiable pour être resoumis.
    </p>

    <div v-if="erreur" class="rounded-lg border border-red-500/30 bg-red-500/10 text-red-400 text-sm px-4 py-3 mb-4">
      {{ erreur }}
    </div>
    <div v-if="succes" class="rounded-lg border border-green-500/30 bg-green-500/10 text-green-400 text-sm px-4 py-3 mb-4">
      {{ succes }}
    </div>

    <div v-if="chargement" class="py-8 text-center" :class="sombre ? 'text-gray-500' : 'text-gray-400'">
      Chargement…
    </div>

    <p
      v-else-if="!episodes.length"
      class="py-8 text-center text-sm"
      :class="sombre ? 'text-gray-500' : 'text-gray-400'"
    >
      Ce programme n'a pas encore d'épisode.
    </p>

    <ul v-else class="space-y-2">
      <li
        v-for="(episode, index) in episodes"
        :key="episode.id"
        class="rounded-lg border p-3 flex flex-wrap items-start gap-3"
        :class="sombre ? 'border-white/10 bg-white/5' : 'border-gray-200 bg-white'"
      >
        <!-- Rang : c'est lui qui pilote la rotation en grille -->
        <div class="flex flex-col gap-1">
          <button
            type="button"
            :disabled="index === 0"
            class="w-7 h-6 rounded text-xs border disabled:opacity-30"
            :class="sombre ? 'border-white/15 text-gray-300 hover:border-yellow-400' : 'border-gray-300 text-gray-600 hover:border-gray-900'"
            title="Monter"
            @click="deplacer(index, -1)"
          >
            ▲
          </button>
          <button
            type="button"
            :disabled="index === episodes.length - 1"
            class="w-7 h-6 rounded text-xs border disabled:opacity-30"
            :class="sombre ? 'border-white/15 text-gray-300 hover:border-yellow-400' : 'border-gray-300 text-gray-600 hover:border-gray-900'"
            title="Descendre"
            @click="deplacer(index, 1)"
          >
            ▼
          </button>
        </div>

        <div class="min-w-0 flex-1">
          <p class="font-semibold" :class="sombre ? 'text-white' : 'text-gray-900'">
            <span v-if="episode.numero_episode" :class="sombre ? 'text-gray-500 font-normal' : 'text-gray-400 font-normal'">
              {{ episode.numero_episode }}.
            </span>
            {{ episode.titre }}
          </p>
          <p class="text-xs mt-1 flex flex-wrap items-center gap-2">
            <span class="rounded-full border px-2 py-0.5" :class="badgeEtat(episode.etat)">
              {{ LIBELLES_ETAT_EPISODE[episode.etat] || episode.etat }}
            </span>
            <span v-if="episode.a_la_une" class="rounded-full border border-yellow-500/30 bg-yellow-500/15 text-yellow-400 px-2 py-0.5">
              À la une
            </span>
            <span v-if="episode.duree_minutes" :class="sombre ? 'text-gray-500' : 'text-gray-500'">
              {{ episode.duree_minutes }} min
            </span>
            <span v-if="!episode.media_url" class="text-red-400">Média manquant</span>
          </p>

          <!-- Le motif fait tout l'intérêt du rejet : sans lui, rien à corriger -->
          <p
            v-if="episode.etat === 'rejete' && episode.motif_rejet"
            class="text-xs mt-2 rounded border border-red-500/30 bg-red-500/10 text-red-300 px-3 py-2"
          >
            Motif du refus : {{ episode.motif_rejet }}
          </p>
        </div>

        <div class="flex flex-wrap gap-2">
          <button
            type="button"
            class="text-xs underline"
            :class="sombre ? 'text-gray-400 hover:text-yellow-400' : 'text-gray-500 hover:text-gray-900'"
            @click="ouvrirEdition(episode)"
          >
            Modifier
          </button>
          <button
            v-if="episode.etat === 'publie'"
            type="button"
            class="text-xs underline"
            :class="sombre ? 'text-gray-400 hover:text-yellow-400' : 'text-gray-500 hover:text-gray-900'"
            @click="basculerALaUne(episode)"
          >
            {{ episode.a_la_une ? 'Retirer de la une' : 'Mettre à la une' }}
          </button>
          <button
            type="button"
            class="text-xs underline text-red-400 hover:text-red-300"
            @click="cibleSuppression = episode"
          >
            Supprimer
          </button>
        </div>
      </li>
    </ul>

    <!-- Formulaire -->
    <div v-if="showFormulaire" class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/70" @click.self="showFormulaire = false">
      <div
        class="w-full max-w-lg max-h-[90vh] overflow-y-auto rounded-xl p-6"
        :class="sombre ? 'bg-neutral-900 border border-white/10' : 'bg-white'"
      >
        <h4 class="font-oswald text-xl font-bold mb-4" :class="sombre ? 'text-white' : 'text-gray-900'">
          {{ enEdition ? 'Modifier l’épisode' : 'Nouvel épisode' }}
        </h4>

        <div v-if="erreur" class="rounded-lg border border-red-500/30 bg-red-500/10 text-red-400 text-sm px-4 py-3 mb-4">
          {{ erreur }}
        </div>

        <div class="space-y-3">
          <div>
            <label class="block text-sm mb-1" :class="sombre ? 'text-gray-300' : 'text-gray-700'">Titre *</label>
            <input v-model="formulaire.titre" type="text" :class="champClasses" placeholder="Titre de l’épisode">
          </div>

          <div>
            <label class="block text-sm mb-1" :class="sombre ? 'text-gray-300' : 'text-gray-700'">{{ libelleMedia }} *</label>
            <input v-model="formulaire.media_url" type="text" :class="champClasses" placeholder="https://…">
            <p v-if="enEdition?.etat === 'publie'" class="text-xs mt-1 text-amber-400">
              Remplacer le média d’un épisode publié le renvoie en validation.
            </p>
          </div>

          <div>
            <label class="block text-sm mb-1" :class="sombre ? 'text-gray-300' : 'text-gray-700'">Image de couverture</label>
            <input v-model="formulaire.image_couverture_url" type="text" :class="champClasses" placeholder="https://…">
          </div>

          <div class="grid grid-cols-2 gap-3">
            <div>
              <label class="block text-sm mb-1" :class="sombre ? 'text-gray-300' : 'text-gray-700'">N° d’épisode</label>
              <input v-model.number="formulaire.numero_episode" type="number" min="1" :class="champClasses">
            </div>
            <div>
              <label class="block text-sm mb-1" :class="sombre ? 'text-gray-300' : 'text-gray-700'">Durée (min)</label>
              <input v-model.number="formulaire.duree_minutes" type="number" min="1" :class="champClasses">
            </div>
          </div>

          <div>
            <label class="block text-sm mb-1" :class="sombre ? 'text-gray-300' : 'text-gray-700'">Description</label>
            <textarea v-model="formulaire.description" rows="3" :class="champClasses" />
          </div>
        </div>

        <div class="flex justify-end gap-3 mt-6">
          <button
            type="button"
            class="text-sm"
            :class="sombre ? 'text-gray-400 hover:text-white' : 'text-gray-500 hover:text-gray-900'"
            @click="showFormulaire = false"
          >
            Annuler
          </button>
          <button
            type="button"
            class="rounded-full px-5 py-2 text-sm font-semibold"
            :class="sombre ? 'bg-yellow-400 text-neutral-900 hover:bg-yellow-300' : 'bg-gray-900 text-white hover:bg-gray-700'"
            @click="enregistrer"
          >
            {{ enEdition ? 'Enregistrer' : (admin ? 'Publier' : 'Soumettre') }}
          </button>
        </div>
      </div>
    </div>

    <!-- Suppression -->
    <div v-if="cibleSuppression" class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/70" @click.self="cibleSuppression = null">
      <div class="w-full max-w-md rounded-xl p-6" :class="sombre ? 'bg-neutral-900 border border-white/10' : 'bg-white'">
        <h4 class="font-bold text-lg mb-2" :class="sombre ? 'text-white' : 'text-gray-900'">
          Supprimer « {{ cibleSuppression.titre }} » ?
        </h4>
        <p class="text-sm mb-6" :class="sombre ? 'text-gray-400' : 'text-gray-600'">
          L’épisode quitte la rotation et l’espace public. Ses réactions et commentaires sont conservés.
        </p>
        <div class="flex justify-end gap-3">
          <button
            type="button"
            class="text-sm"
            :class="sombre ? 'text-gray-400 hover:text-white' : 'text-gray-500 hover:text-gray-900'"
            @click="cibleSuppression = null"
          >
            Annuler
          </button>
          <button type="button" class="rounded-full bg-red-500 text-white px-5 py-2 text-sm font-semibold hover:bg-red-400" @click="executerSuppression">
            Supprimer
          </button>
        </div>
      </div>
    </div>
  </section>
</template>
