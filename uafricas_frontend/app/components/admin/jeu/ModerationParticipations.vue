<script setup lang="ts">
/**
 * Modération des photos de concours (feature 014) : la file transversale et
 * l'onglet d'un concours partagent ce composant. Une photo s'affiche en grand,
 * c'est sur elle qu'on décide.
 */
import {
  LIBELLES_ETAT_PARTICIPATION,
  messageErreurConcours,
  type EtatParticipation,
  type ParticipationModerationAPI,
} from '~/composables/useAdminConcours'

const props = defineProps<{ concoursId?: string }>()
const emit = defineEmits<{ change: [] }>()

const { listerParticipations, accepter, rejeter, modererGroupe, retablir } = useAdminConcours()

const etat = ref<EtatParticipation | 'toutes'>('en_attente')
const elements = ref<ParticipationModerationAPI[]>([])
const total = ref(0)
const page = ref(1)
const chargement = ref(true)
const enCours = ref(false)
const erreur = ref('')
const message = ref('')
const selection = ref<Set<string>>(new Set())
/** Participation dont on saisit le motif de rejet (`'groupe'` pour la sélection). */
const rejetEnCours = ref<string | null>(null)
const motif = ref('')

const charger = async () => {
  erreur.value = ''
  try {
    const res = await listerParticipations({ etat: etat.value, concours: props.concoursId, page: page.value })
    elements.value = res?.data ?? []
    total.value = res?.total ?? 0
    selection.value = new Set()
  }
  catch (e) {
    erreur.value = messageErreurConcours(e, 'Impossible de charger les participations.')
  }
  finally {
    chargement.value = false
  }
}

const agir = async (action: () => Promise<unknown>, texte: string) => {
  enCours.value = true
  erreur.value = ''
  try {
    await action()
    message.value = texte
    setTimeout(() => { if (message.value === texte) message.value = '' }, 4000)
    rejetEnCours.value = null
    motif.value = ''
    await charger()
    emit('change')
  }
  catch (e) {
    erreur.value = messageErreurConcours(e)
  }
  finally {
    enCours.value = false
  }
}

const confirmerRejet = () => {
  const m = motif.value.trim()
  if (!m) {
    erreur.value = 'Le motif du rejet est obligatoire : le membre le recevra.'
    return
  }
  if (rejetEnCours.value === 'groupe') {
    const ids = [...selection.value]
    agir(() => modererGroupe(ids, 'rejeter', m), `${ids.length} photo(s) refusée(s).`)
  }
  else if (rejetEnCours.value) {
    const id = rejetEnCours.value
    agir(() => rejeter(id, m), 'Photo refusée, le membre est prévenu.')
  }
}

const basculer = (id: string) => {
  const s = new Set(selection.value)
  if (s.has(id)) s.delete(id)
  else s.add(id)
  selection.value = s
}

watch(etat, () => { page.value = 1; chargement.value = true; charger() })
watch(page, charger)
onMounted(charger)

const CLASSES_ETAT: Record<EtatParticipation, string> = {
  en_attente: 'badge-warning',
  publiee: 'badge-success',
  rejetee: 'badge-error',
  retiree: 'badge-ghost',
  suspendue: 'badge-error',
}
const dateLisible = (iso: string) =>
  new Date(iso).toLocaleString('fr-FR', { day: 'numeric', month: 'short', hour: '2-digit', minute: '2-digit' })
</script>

<template>
  <div>
    <div class="mb-4 flex flex-wrap items-center gap-3">
      <select v-model="etat" class="select select-bordered select-sm" aria-label="État des participations">
        <option value="en_attente">En attente</option>
        <option value="publiee">Publiées</option>
        <option value="suspendue">Suspendues</option>
        <option value="rejetee">Refusées</option>
        <option value="toutes">Toutes</option>
      </select>
      <span class="text-sm text-base-content/60">{{ total }} photo(s)</span>
      <div v-if="etat === 'en_attente' && selection.size" class="ml-auto flex gap-2">
        <button type="button" class="btn btn-success btn-sm" :disabled="enCours" @click="agir(() => modererGroupe([...selection], 'accepter'), `${selection.size} photo(s) publiée(s).`)">
          Accepter la sélection ({{ selection.size }})
        </button>
        <button type="button" class="btn btn-outline btn-error btn-sm" :disabled="enCours" @click="rejetEnCours = 'groupe'">
          Refuser la sélection
        </button>
      </div>
    </div>

    <div v-if="message" class="alert alert-success mb-4 text-sm">{{ message }}</div>
    <div v-if="erreur" class="alert alert-error mb-4 text-sm" role="alert">{{ erreur }}</div>

    <div v-if="rejetEnCours" class="card mb-4 bg-base-200">
      <div class="card-body gap-3">
        <label class="flex flex-col">
          <span class="label-text mb-1 font-medium">
            Motif du refus {{ rejetEnCours === 'groupe' ? `(${selection.size} photos)` : '' }} *
          </span>
          <input v-model="motif" type="text" class="input input-bordered input-sm w-full" placeholder="Hors sujet, photo floue, visage d'un mineur…" @keyup.enter="confirmerRejet">
        </label>
        <div class="flex gap-2">
          <button type="button" class="btn btn-error btn-sm" :disabled="enCours" @click="confirmerRejet">Refuser</button>
          <button type="button" class="btn btn-ghost btn-sm" @click="rejetEnCours = null; motif = ''">Annuler</button>
        </div>
      </div>
    </div>

    <div v-if="chargement" class="skeleton h-64 w-full" />
    <p v-else-if="!elements.length" class="py-10 text-center text-base-content/60">Aucune photo dans cet état.</p>

    <div v-else class="grid gap-4 sm:grid-cols-2 xl:grid-cols-3">
      <article v-for="p in elements" :key="p.id" class="card border border-base-300 bg-base-100">
        <figure class="bg-base-200">
          <img :src="urlMedia(p.media_url) ?? undefined" :alt="p.legende ?? 'Photo de concours'" class="h-60 w-full object-contain">
        </figure>
        <div class="card-body gap-2 p-4">
          <div class="flex items-center gap-2">
            <input
              v-if="p.etat === 'en_attente'"
              type="checkbox"
              class="checkbox checkbox-sm"
              :checked="selection.has(p.id)"
              :aria-label="`Sélectionner la photo de ${p.auteur_prenom} ${p.auteur_nom}`"
              @change="basculer(p.id)"
            >
            <span class="badge badge-sm" :class="CLASSES_ETAT[p.etat]">{{ LIBELLES_ETAT_PARTICIPATION[p.etat] }}</span>
            <span v-if="p.nombre_signalements" class="badge badge-error badge-outline badge-sm">{{ p.nombre_signalements }} signalement(s)</span>
          </div>
          <p v-if="p.legende" class="text-sm">« {{ p.legende }} »</p>
          <p class="text-xs text-base-content/60">
            {{ p.auteur_prenom }} {{ p.auteur_nom }} · {{ dateLisible(p.created_at) }}
            <template v-if="!concoursId"> · <NuxtLink :to="`/admin/activites/concours/${p.concours_id}`" class="link">{{ p.concours_titre }}</NuxtLink></template>
          </p>
          <p v-if="p.motif_rejet" class="text-xs text-error">Motif : {{ p.motif_rejet }}</p>
          <div class="card-actions mt-1">
            <template v-if="p.etat === 'en_attente'">
              <button type="button" class="btn btn-success btn-xs" :disabled="enCours" @click="agir(() => accepter(p.id), 'Photo publiée, le membre est prévenu.')">Accepter</button>
              <button type="button" class="btn btn-outline btn-error btn-xs" :disabled="enCours" @click="rejetEnCours = p.id">Refuser</button>
            </template>
            <button v-else-if="p.etat === 'publiee'" type="button" class="btn btn-outline btn-error btn-xs" :disabled="enCours" @click="rejetEnCours = p.id">Retirer du concours</button>
            <button v-else-if="p.etat === 'suspendue'" type="button" class="btn btn-outline btn-xs" :disabled="enCours" @click="agir(() => retablir(p.id), 'Photo rétablie.')">Rétablir</button>
          </div>
        </div>
      </article>
    </div>

    <div v-if="total > elements.length" class="join mt-6">
      <button type="button" class="btn join-item btn-sm" :disabled="page <= 1" @click="page--">Précédent</button>
      <button type="button" class="btn join-item btn-sm" disabled>Page {{ page }}</button>
      <button type="button" class="btn join-item btn-sm" :disabled="page * 24 >= total" @click="page++">Suivant</button>
    </div>
  </div>
</template>
