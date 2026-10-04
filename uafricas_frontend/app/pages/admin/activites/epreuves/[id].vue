<script setup lang="ts">
/**
 * Saisie et édition d'une épreuve (feature 013).
 * `/admin/activites/epreuves/nouvelle` crée ; tout autre segment édite.
 *
 * Le refus de publication affiche le message du serveur tel quel : c'est lui
 * qui nomme ce qui manque (explication, propositions en double…).
 */
import {
  CLASSES_ETAT_EPREUVE,
  LIBELLES_ETAT_EPREUVE,
  LIBELLES_SOURCE_ETAT,
  messageErreurAdminJeu,
  type EpreuveAdminAPI,
  type EpreuveForm,
  type ModuleAdminAPI,
} from '~/composables/useAdminJeu'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const route = useRoute()
const id = computed(() => String(route.params.id))
const creation = computed(() => id.value === 'nouvelle')

const {
  obtenirEpreuve, creerEpreuve, modifierEpreuve, publierEpreuve, retirerEpreuve, listerModules,
} = useAdminJeu()

const formVide = (): EpreuveForm => ({
  module: '',
  enonce: '',
  media_type: null,
  media_url: null,
  type_reponse: 'choix',
  propositions: ['', '', '', ''],
  bonne_reponse: 1,
  reponse_pays_iso: null,
  elements: [],
  paires: [],
  explication: null,
  difficulte: 1,
  theme: null,
  pays_id: null,
  type_source: null,
  source_id: null,
})

const form = ref<EpreuveForm>(formVide())
const epreuve = ref<EpreuveAdminAPI | null>(null)
const modules = ref<ModuleAdminAPI[]>([])
const chargement = ref(true)
const enCours = ref(false)
const erreur = ref('')
const message = ref('')

/** Une épreuve retirée ou rejetée ne se modifie plus. */
const figee = computed(() => epreuve.value != null && ['retiree', 'rejetee'].includes(epreuve.value.etat))
const publiable = computed(() => epreuve.value != null && ['candidate', 'a_revoir'].includes(epreuve.value.etat))

const remplir = (e: EpreuveAdminAPI) => {
  epreuve.value = e
  form.value = {
    module: e.module_code,
    enonce: e.enonce,
    media_type: e.media_type,
    media_url: e.media_url,
    type_reponse: e.type_reponse,
    // Un type autre que « choix » n'a pas de propositions à éditer ici : on
    // garde des lignes vides si l'on rebascule sur le choix multiple.
    propositions: e.type_reponse === 'choix' ? [...e.propositions] : ['', '', '', ''],
    bonne_reponse: e.bonne_reponse ?? 1,
    reponse_pays_iso: e.reponse_pays_iso,
    // La base garde l'ordre et les paires mélangés : on édite leur forme attendue.
    elements: (e.elements_attendus ?? []).map(x => ({ ...x })),
    paires: (e.paires_attendues ?? []).map(x => ({ ...x })),
    explication: e.explication,
    difficulte: e.difficulte,
    theme: e.theme,
    pays_id: e.pays_id,
    type_source: e.type_source,
    source_id: e.source_id,
  }
}

const notifier = (texte: string) => {
  message.value = texte
  setTimeout(() => { if (message.value === texte) message.value = '' }, 4000)
}

/** Exécute une action en gérant l'attente et l'erreur au même endroit. */
const executer = async (action: () => Promise<void>) => {
  erreur.value = ''
  enCours.value = true
  try {
    await action()
  }
  catch (e) {
    erreur.value = messageErreurAdminJeu(e)
  }
  finally {
    enCours.value = false
  }
}

const enregistrer = () => executer(async () => {
  if (creation.value) {
    const creee = await creerEpreuve(form.value)
    if (creee) await navigateTo(`/admin/activites/epreuves/${creee.id}`)
    return
  }
  const modifiee = await modifierEpreuve(id.value, form.value)
  if (modifiee) remplir(modifiee)
  notifier('Épreuve enregistrée.')
})

// Publier enregistre d'abord : sinon on publierait la version en base, pas
// celle que l'administrateur a sous les yeux.
const publier = () => executer(async () => {
  await modifierEpreuve(id.value, form.value)
  const publiee = await publierEpreuve(id.value)
  if (publiee) remplir(publiee)
  notifier('Épreuve publiée : elle peut être servie.')
})

const retirer = () => executer(async () => {
  if (!confirm('Retirer cette épreuve ? Elle ne sera plus servie. Les scores déjà gagnés dessus restent acquis.')) return
  const retiree = await retirerEpreuve(id.value)
  if (retiree) remplir(retiree)
  notifier('Épreuve retirée.')
})

const charger = async () => {
  chargement.value = true
  erreur.value = ''
  try {
    modules.value = await listerModules()
    if (creation.value) {
      epreuve.value = null
      form.value = { ...formVide(), module: modules.value[0]?.code ?? '' }
    }
    else {
      const e = await obtenirEpreuve(id.value)
      if (e) remplir(e)
    }
  }
  catch (e) {
    erreur.value = messageErreurAdminJeu(e, 'Épreuve introuvable.')
  }
  finally {
    chargement.value = false
  }
}

onMounted(charger)
watch(id, charger)
</script>

<template>
  <div>
    <AdminPageHeader
      :titre="creation ? 'Nouvelle épreuve' : 'Épreuve'"
      :sous-titre="creation ? 'Elle naît en brouillon ; vous la publiez ensuite.' : undefined"
    >
      <template #actions>
        <NuxtLink to="/admin/activites/epreuves" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Retour à la liste
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div v-if="chargement" class="flex justify-center py-16">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <div v-else class="space-y-4">
      <div v-if="message" class="alert alert-success text-sm">{{ message }}</div>
      <div v-if="erreur" class="alert alert-error text-sm" role="alert">{{ erreur }}</div>

      <!-- État, origine et usage de l'épreuve existante -->
      <div v-if="epreuve" class="flex flex-wrap items-center gap-3 text-sm">
        <span class="badge" :class="CLASSES_ETAT_EPREUVE[epreuve.etat]">
          {{ LIBELLES_ETAT_EPREUVE[epreuve.etat] }}
        </span>
        <span class="badge badge-outline">{{ epreuve.origine === 'derivee' ? 'Dérivée' : 'Saisie' }}</span>
        <span v-if="epreuve.type_source" class="text-base-content/70">
          {{ LIBELLES_SOURCE_ETAT[epreuve.source_etat] }} ({{ epreuve.type_source }})
        </span>
        <span class="text-base-content/70">
          Servie {{ epreuve.nombre_servie }} fois<template v-if="epreuve.taux_reussite != null">,
            {{ Math.round(epreuve.taux_reussite * 100) }} % de réussite</template>
        </span>
        <span v-if="epreuve.motif_rejet" class="text-error">Motif du rejet : {{ epreuve.motif_rejet }}</span>
      </div>

      <form class="card bg-base-100 shadow-sm" @submit.prevent="enregistrer">
        <div class="card-body">
          <AdminJeuEpreuveFormulaire v-model="form" :modules="modules" :desactive="figee" />

          <div class="mt-6 flex flex-wrap items-center justify-end gap-3 border-t border-base-200 pt-4">
            <button
              v-if="epreuve?.etat === 'jouable'"
              type="button"
              class="btn btn-outline btn-error btn-sm mr-auto"
              :disabled="enCours"
              @click="retirer"
            >
              Retirer du jeu
            </button>
            <button v-if="!figee" type="submit" class="btn btn-sm" :class="publiable ? 'btn-outline' : 'btn-primary'" :disabled="enCours">
              {{ creation ? 'Créer le brouillon' : 'Enregistrer' }}
            </button>
            <button
              v-if="publiable"
              type="button"
              class="btn btn-primary btn-sm"
              :disabled="enCours"
              @click="publier"
            >
              Enregistrer et publier
            </button>
          </div>
        </div>
      </form>
    </div>
  </div>
</template>
