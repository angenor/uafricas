<script setup lang="ts">
/**
 * Fiche d'un concours (feature 014). `/admin/activites/concours/nouveau`
 * crée ; tout autre segment édite. Les onglets « Jury » et « Suivi du vote »
 * viennent avec les paliers 5 et 6.
 */
import {
  LIBELLES_PHASE,
  messageErreurConcours,
  type ConcoursAdminAPI,
  type ConcoursForm,
} from '~/composables/useAdminConcours'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const route = useRoute()
const id = computed(() => String(route.params.id))
const creation = computed(() => id.value === 'nouveau')

const { obtenir, creer, modifier, supprimer, annuler } = useAdminConcours()

const dans = (heures: number) => new Date(Date.now() + heures * 3600 * 1000).toISOString()
const formVide = (): ConcoursForm => ({
  format: 'photo',
  titre: '',
  theme: '',
  reglement: '',
  rattachement: null,
  image_url: null,
  appel_debut: dans(1),
  vote_debut: dans(24 * 7),
  vote_fin: dans(24 * 10),
  participations_max: 1,
  minimum_participations: 4,
  votes_max: null,
  presentations_min: 10,
  jury: false,
  jury_finalistes: 10,
  jury_delai_jours: 7,
  prime_participation: null,
  prime_podium: null,
})

const form = ref<ConcoursForm>(formVide())
const concours = ref<ConcoursAdminAPI | null>(null)
const onglet = ref<'fiche' | 'participations'>('fiche')
const chargement = ref(true)
const enCours = ref(false)
const erreur = ref('')
const message = ref('')
const motifAnnulation = ref('')
const annulationOuverte = ref(false)

const remplir = (c: ConcoursAdminAPI) => {
  concours.value = c
  form.value = {
    format: c.format, titre: c.titre, theme: c.theme, reglement: c.reglement,
    rattachement: c.rattachement, image_url: c.image_url,
    appel_debut: c.appel_debut, vote_debut: c.vote_debut, vote_fin: c.vote_fin,
    participations_max: c.participations_max, minimum_participations: c.minimum_participations,
    votes_max: c.votes_max, presentations_min: c.presentations_min, jury: c.jury,
    jury_finalistes: c.jury_finalistes, jury_delai_jours: c.jury_delai_jours,
    prime_participation: c.prime_participation, prime_podium: c.prime_podium,
  }
}

/** Un champ numérique vidé vaut `''` : on l'envoie `null`. */
const corps = (): ConcoursForm => {
  const f = { ...form.value }
  for (const cle of ['votes_max', 'prime_participation'] as const) {
    if ((f[cle] as unknown) === '') f[cle] = null
  }
  return f
}

const notifier = (texte: string) => {
  message.value = texte
  setTimeout(() => { if (message.value === texte) message.value = '' }, 5000)
}

const enregistrer = async () => {
  enCours.value = true
  erreur.value = ''
  try {
    if (creation.value) {
      const c = await creer(corps())
      if (c) await navigateTo(`/admin/activites/concours/${c.id}`)
    }
    else {
      const c = await modifier(id.value, corps())
      if (c) { remplir(c); notifier('Concours enregistré.') }
    }
  }
  catch (e) {
    erreur.value = messageErreurConcours(e, 'Le concours n\'a pas pu être enregistré.')
  }
  finally {
    enCours.value = false
  }
}

const confirmerAnnulation = async () => {
  if (!motifAnnulation.value.trim()) {
    erreur.value = 'Le motif de l\'annulation est obligatoire : les participants le recevront.'
    return
  }
  enCours.value = true
  try {
    const c = await annuler(id.value, motifAnnulation.value.trim())
    if (c) { remplir(c); annulationOuverte.value = false; notifier('Concours annulé, les participants sont prévenus.') }
  }
  catch (e) {
    erreur.value = messageErreurConcours(e)
  }
  finally {
    enCours.value = false
  }
}

const supprimerConcours = async () => {
  if (!confirm('Supprimer ce concours ? Il n\'a pas encore ouvert son appel.')) return
  try {
    await supprimer(id.value)
    await navigateTo('/admin/activites/concours')
  }
  catch (e) {
    erreur.value = messageErreurConcours(e)
  }
}

const recharger = async () => {
  if (creation.value) return
  const c = await obtenir(id.value)
  if (c) remplir(c)
}

onMounted(async () => {
  try {
    await recharger()
  }
  catch (e) {
    erreur.value = messageErreurConcours(e, 'Concours introuvable.')
  }
  finally {
    chargement.value = false
  }
})
</script>

<template>
  <div>
    <AdminPageHeader
      :titre="creation ? 'Nouveau concours' : (concours?.titre ?? 'Concours')"
      :sous-titre="concours ? LIBELLES_PHASE[concours.phase] : 'Bataille de photos'"
    >
      <template #actions>
        <NuxtLink to="/admin/activites/concours" class="btn btn-ghost btn-sm">Retour à la liste</NuxtLink>
        <NuxtLink v-if="concours" :to="`/activites/concours/${concours.id}`" target="_blank" class="btn btn-ghost btn-sm">Page publique</NuxtLink>
      </template>
    </AdminPageHeader>

    <div v-if="message" class="alert alert-success mb-4 text-sm">{{ message }}</div>
    <div v-if="erreur" class="alert alert-error mb-4 text-sm" role="alert">{{ erreur }}</div>
    <div v-if="chargement" class="skeleton h-64 w-full" />

    <template v-else>
      <div v-if="concours" class="stats stats-vertical mb-6 w-full shadow-sm lg:stats-horizontal">
        <div class="stat"><div class="stat-title">À modérer</div><div class="stat-value text-warning">{{ concours.en_attente }}</div></div>
        <div class="stat"><div class="stat-title">Publiées</div><div class="stat-value">{{ concours.publiees }}</div><div class="stat-desc">minimum {{ concours.minimum_participations }} pour ouvrir le vote</div></div>
        <div class="stat"><div class="stat-title">Votes</div><div class="stat-value">{{ concours.votes }}</div><div class="stat-desc">{{ concours.votes_comptes }} comptés</div></div>
      </div>

      <p v-if="concours?.motif_annulation" class="alert alert-warning mb-4 text-sm">Annulé : {{ concours.motif_annulation }}</p>

      <div v-if="concours" role="tablist" class="tabs tabs-bordered mb-4">
        <button type="button" role="tab" class="tab" :class="onglet === 'fiche' && 'tab-active'" @click="onglet = 'fiche'">Fiche</button>
        <button type="button" role="tab" class="tab" :class="onglet === 'participations' && 'tab-active'" @click="onglet = 'participations'">
          Participations <span v-if="concours.en_attente" class="badge badge-warning badge-sm ml-2">{{ concours.en_attente }}</span>
        </button>
      </div>

      <form v-if="onglet === 'fiche'" class="card bg-base-100 shadow-sm" @submit.prevent="enregistrer">
        <div class="card-body gap-6">
          <AdminJeuConcoursFormulaire v-model="form" :phase="concours?.phase ?? null" />

          <div v-if="annulationOuverte" class="flex flex-col gap-2 rounded-lg bg-base-200 p-4">
            <label class="flex flex-col">
              <span class="label-text mb-1 font-medium">Motif de l'annulation *</span>
              <input v-model="motifAnnulation" type="text" class="input input-bordered input-sm w-full" placeholder="Les participants recevront ce motif.">
            </label>
            <div class="flex gap-2">
              <button type="button" class="btn btn-error btn-sm" :disabled="enCours" @click="confirmerAnnulation">Annuler le concours</button>
              <button type="button" class="btn btn-ghost btn-sm" @click="annulationOuverte = false">Garder le concours</button>
            </div>
          </div>

          <div class="flex flex-wrap justify-end gap-2">
            <button v-if="concours?.phase === 'a_venir'" type="button" class="btn btn-outline btn-error btn-sm mr-auto" @click="supprimerConcours">Supprimer</button>
            <button
              v-else-if="concours && ['appel', 'vote', 'deliberation'].includes(concours.phase)"
              type="button"
              class="btn btn-outline btn-error btn-sm mr-auto"
              @click="annulationOuverte = true"
            >
              Annuler le concours…
            </button>
            <button
              v-if="!concours || !['resultats', 'annule'].includes(concours.phase)"
              type="submit"
              class="btn btn-primary btn-sm"
              :disabled="enCours"
            >
              {{ creation ? 'Programmer le concours' : 'Enregistrer' }}
            </button>
          </div>
        </div>
      </form>

      <AdminJeuModerationParticipations v-else-if="concours" :concours-id="concours.id" @change="recharger" />
    </template>
  </div>
</template>
