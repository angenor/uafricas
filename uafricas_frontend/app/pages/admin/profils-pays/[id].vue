<script setup lang="ts">
import type { StatsCardData } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const route = useRoute()
const id = route.params.id as string

const {
  ficheDetail,
  regions, groupesEthniques, alliances, contes,
  sitesTouristiques, secteurs, saisons, liensInterethniques,
  chargerDetail, modifier, debloquer,
  chargerRegions, creerRegion, modifierRegion, supprimerRegion,
  chargerGroupesEthniques, creerGroupeEthnique, modifierGroupeEthnique, supprimerGroupeEthnique,
  chargerAlliances, creerAlliance, modifierAlliance, supprimerAlliance,
  chargerContes, creerConte, modifierConte, supprimerConte,
  chargerSitesTouristiques, creerSiteTouristique, modifierSiteTouristique, supprimerSiteTouristique, definirVerificationSite,
  chargerSecteurs, creerSecteur, modifierSecteur, supprimerSecteur,
  chargerSaisons, creerSaison, modifierSaison, supprimerSaison,
  chargerLiensInterethniques, creerLienInterethnique, modifierLienInterethnique, supprimerLienInterethnique,
  loading, error,
} = useAdminProfilsPays()

// ── Onglets ─────────────────────────────────────────────────

type TabType = 'general' | 'regions' | 'groupes' | 'alliances' | 'contes' | 'sites' | 'secteurs' | 'saisons' | 'liens'

const ongletActif = ref<TabType>('general')

const tabs: Array<{ id: TabType; label: string; icon: string; countKey?: string }> = [
  { id: 'general', label: 'General', icon: 'circle-info' },
  { id: 'regions', label: 'Regions', icon: 'location-dot', countKey: 'nb_regions' },
  { id: 'groupes', label: 'Ethnies', icon: 'users', countKey: 'nb_groupes_ethniques' },
  { id: 'alliances', label: 'Alliances', icon: 'handshake', countKey: 'nb_alliances' },
  { id: 'contes', label: 'Contes', icon: 'book-open', countKey: 'nb_contes' },
  { id: 'sites', label: 'Sites', icon: 'map-marker-alt', countKey: 'nb_sites_touristiques' },
  { id: 'secteurs', label: 'Secteurs', icon: 'building', countKey: 'nb_secteurs' },
  { id: 'saisons', label: 'Saisons', icon: 'calendar-days', countKey: 'nb_saisons' },
  { id: 'liens', label: 'Liens', icon: 'link', countKey: 'nb_liens_interethniques' },
]

const getCount = (key: string) => {
  if (!ficheDetail.value) return 0
  return (ficheDetail.value as any)[key] ?? 0
}

// ── General form ────────────────────────────────────────────

const saving = ref(false)
const erreurLocale = ref<string | null>(null)
const successMsg = ref<string | null>(null)

const form = reactive({
  slogan: '',
  superficie_km2: null as number | null,
  population: null as number | null,
  biographie: '',
  contexte: '',
  contexte_historique: '',
  image_couverture_url: '',
  image_drapeau_url: '',
  image_embleme_url: '',
  image_devise_url: '',
  hymne_national: '',
  langue_officielle: '',
  langues_populaires: '',
  monnaie: '',
  fuseau_horaire: '',
})

const remplirForm = () => {
  if (!ficheDetail.value) return
  const f = ficheDetail.value
  form.slogan = f.slogan || ''
  form.superficie_km2 = f.superficie_km2
  form.population = f.population
  form.biographie = f.biographie || ''
  form.contexte = f.contexte || ''
  form.contexte_historique = f.contexte_historique || ''
  form.image_couverture_url = f.image_couverture_url || ''
  form.image_drapeau_url = f.image_drapeau_url || ''
  form.image_embleme_url = f.image_embleme_url || ''
  form.image_devise_url = f.image_devise_url || ''
  form.hymne_national = f.hymne_national || ''
  form.langue_officielle = f.langue_officielle || ''
  form.langues_populaires = f.langues_populaires || ''
  form.monnaie = f.monnaie || ''
  form.fuseau_horaire = f.fuseau_horaire || ''
}

const sauvegarderGeneral = async () => {
  saving.value = true
  erreurLocale.value = null
  successMsg.value = null
  try {
    const body: Record<string, any> = {}
    if (form.slogan.trim()) body.slogan = form.slogan.trim()
    if (form.superficie_km2 !== null) body.superficie_km2 = form.superficie_km2
    if (form.population !== null) body.population = form.population
    if (form.biographie.trim()) body.biographie = form.biographie.trim()
    if (form.contexte.trim()) body.contexte = form.contexte.trim()
    if (form.contexte_historique.trim()) body.contexte_historique = form.contexte_historique.trim()
    if (form.image_couverture_url.trim()) body.image_couverture_url = form.image_couverture_url.trim()
    if (form.image_drapeau_url.trim()) body.image_drapeau_url = form.image_drapeau_url.trim()
    if (form.image_embleme_url.trim()) body.image_embleme_url = form.image_embleme_url.trim()
    if (form.image_devise_url.trim()) body.image_devise_url = form.image_devise_url.trim()
    if (form.hymne_national.trim()) body.hymne_national = form.hymne_national.trim()
    if (form.langue_officielle.trim()) body.langue_officielle = form.langue_officielle.trim()
    if (form.langues_populaires.trim()) body.langues_populaires = form.langues_populaires.trim()
    if (form.monnaie.trim()) body.monnaie = form.monnaie.trim()
    if (form.fuseau_horaire.trim()) body.fuseau_horaire = form.fuseau_horaire.trim()

    await modifier(id, body)
    successMsg.value = 'Fiche territoire mise a jour'
    setTimeout(() => { successMsg.value = null }, 3000)
    await chargerDetail(id)
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  } finally { saving.value = false }
}

// ── Déblocage (modération communautaire) ────────────────────
const debloquage = ref(false)

const debloquerFiche = async () => {
  if (!ficheDetail.value?.bloquee) return
  if (!confirm('Débloquer ce territoire ? Les signalements seront purgés et le compteur remis à zéro.')) return
  debloquage.value = true
  erreurLocale.value = null
  try {
    await debloquer(id)
    successMsg.value = 'Territoire débloqué : il est de nouveau visible publiquement.'
    setTimeout(() => { successMsg.value = null }, 3000)
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors du déblocage'
  } finally { debloquage.value = false }
}

// ── Sous-entites : Etat partage ─────────────────────────────

const showModal = ref(false)
const editMode = ref(false)
const editId = ref('')
const modalLoading = ref(false)
const modalError = ref<string | null>(null)

const showDelete = ref(false)
const deleteTarget = ref<{ id: string; nom: string } | null>(null)
const deleteLoading = ref(false)

// ── Formulaires par sous-entite ─────────────────────────────

const formRegion = reactive({ nom: '', chef_lieu: '', description: '', population: null as number | null })
const formGroupe = reactive({ nom: '', description: '', objets_culturels_distinctifs: '', population_estimee: '', langues: '', region_id: '' })
const formAlliance = reactive({ nom: '', description: '', groupes_impliques: '', signification: '' })
const formConte = reactive({ titre: '', contenu: '', type: '', groupe_ethnique_id: '', image_url: '' })
const formSite = reactive({ nom: '', description: '', image_url: '', longitude: null as number | null, latitude: null as number | null, region_id: '' })
const formSecteur = reactive({ nom: '', description: '' })
const formSaison = reactive({ nom: '', description: '', mois_debut: null as number | null, mois_fin: null as number | null })
const formLien = reactive({ pays_lie_id: '', description: '', type_lien: '' })

// Contes filter
const filtreTypeConte = ref('')

const moisNoms = ['Janvier', 'Fevrier', 'Mars', 'Avril', 'Mai', 'Juin', 'Juillet', 'Aout', 'Septembre', 'Octobre', 'Novembre', 'Decembre']

const modalTitres: Record<TabType, string> = {
  general: '',
  regions: 'Region',
  groupes: 'Groupe ethnique',
  alliances: 'Alliance interethnique',
  contes: 'Conte / Histoire',
  sites: 'Site touristique',
  secteurs: 'Secteur de developpement',
  saisons: 'Saison',
  liens: 'Lien interethnique',
}

// ── Helpers ─────────────────────────────────────────────────

const ouvrirAjout = () => {
  editMode.value = false
  editId.value = ''
  modalError.value = null
  resetForm()
  showModal.value = true
}

const ouvrirEdition = (item: any) => {
  editMode.value = true
  editId.value = item.id
  modalError.value = null
  remplirFormSousEntite(item)
  showModal.value = true
}

const resetForm = () => {
  switch (ongletActif.value) {
    case 'regions': Object.assign(formRegion, { nom: '', chef_lieu: '', description: '', population: null }); break
    case 'groupes': Object.assign(formGroupe, { nom: '', description: '', objets_culturels_distinctifs: '', population_estimee: '', langues: '', region_id: '' }); break
    case 'alliances': Object.assign(formAlliance, { nom: '', description: '', groupes_impliques: '', signification: '' }); break
    case 'contes': Object.assign(formConte, { titre: '', contenu: '', type: '', groupe_ethnique_id: '', image_url: '' }); break
    case 'sites': Object.assign(formSite, { nom: '', description: '', image_url: '', longitude: null, latitude: null, region_id: '' }); break
    case 'secteurs': Object.assign(formSecteur, { nom: '', description: '' }); break
    case 'saisons': Object.assign(formSaison, { nom: '', description: '', mois_debut: null, mois_fin: null }); break
    case 'liens': Object.assign(formLien, { pays_lie_id: '', description: '', type_lien: '' }); break
  }
}

const remplirFormSousEntite = (item: any) => {
  switch (ongletActif.value) {
    case 'regions': Object.assign(formRegion, { nom: item.nom, chef_lieu: item.chef_lieu || '', description: item.description || '', population: item.population }); break
    case 'groupes': Object.assign(formGroupe, { nom: item.nom, description: item.description || '', objets_culturels_distinctifs: item.objets_culturels_distinctifs || '', population_estimee: item.population_estimee || '', langues: item.langues || '', region_id: item.region_id || '' }); break
    case 'alliances': Object.assign(formAlliance, { nom: item.nom, description: item.description || '', groupes_impliques: item.groupes_impliques || '', signification: item.signification || '' }); break
    case 'contes': Object.assign(formConte, { titre: item.titre, contenu: item.contenu || '', type: item.type_conte || '', groupe_ethnique_id: item.groupe_ethnique_id || '', image_url: item.image_url || '' }); break
    case 'sites': Object.assign(formSite, { nom: item.nom, description: item.description || '', image_url: item.image_url || '', longitude: item.longitude, latitude: item.latitude, region_id: item.region_id || '' }); break
    case 'secteurs': Object.assign(formSecteur, { nom: item.nom, description: item.description || '' }); break
    case 'saisons': Object.assign(formSaison, { nom: item.nom, description: item.description || '', mois_debut: item.mois_debut, mois_fin: item.mois_fin }); break
    case 'liens': Object.assign(formLien, { pays_lie_id: item.pays_lie_id || '', description: item.description || '', type_lien: item.type_lien || '' }); break
  }
}

const buildBody = (): Record<string, any> => {
  const body: Record<string, any> = {}
  const addIfVal = (key: string, val: any) => {
    if (val !== null && val !== undefined && val !== '') body[key] = typeof val === 'string' ? val.trim() : val
  }
  switch (ongletActif.value) {
    case 'regions':
      addIfVal('nom', formRegion.nom); addIfVal('chef_lieu', formRegion.chef_lieu)
      addIfVal('description', formRegion.description); addIfVal('population', formRegion.population)
      break
    case 'groupes':
      addIfVal('nom', formGroupe.nom); addIfVal('description', formGroupe.description)
      addIfVal('objets_culturels_distinctifs', formGroupe.objets_culturels_distinctifs)
      addIfVal('population_estimee', formGroupe.population_estimee); addIfVal('langues', formGroupe.langues)
      addIfVal('region_id', formGroupe.region_id)
      break
    case 'alliances':
      addIfVal('nom', formAlliance.nom); addIfVal('description', formAlliance.description)
      addIfVal('groupes_impliques', formAlliance.groupes_impliques); addIfVal('signification', formAlliance.signification)
      break
    case 'contes':
      addIfVal('titre', formConte.titre); addIfVal('contenu', formConte.contenu)
      addIfVal('type', formConte.type); addIfVal('groupe_ethnique_id', formConte.groupe_ethnique_id)
      addIfVal('image_url', formConte.image_url)
      break
    case 'sites':
      addIfVal('nom', formSite.nom); addIfVal('description', formSite.description)
      addIfVal('image_url', formSite.image_url); addIfVal('longitude', formSite.longitude)
      addIfVal('latitude', formSite.latitude); addIfVal('region_id', formSite.region_id)
      break
    case 'secteurs':
      addIfVal('nom', formSecteur.nom); addIfVal('description', formSecteur.description)
      break
    case 'saisons':
      addIfVal('nom', formSaison.nom); addIfVal('description', formSaison.description)
      addIfVal('mois_debut', formSaison.mois_debut); addIfVal('mois_fin', formSaison.mois_fin)
      break
    case 'liens':
      addIfVal('pays_lie_id', formLien.pays_lie_id); addIfVal('description', formLien.description)
      addIfVal('type_lien', formLien.type_lien)
      break
  }
  return body
}

// ── CRUD generique ──────────────────────────────────────────

const sauvegarderSousEntite = async () => {
  modalLoading.value = true
  modalError.value = null
  try {
    const body = buildBody()
    switch (ongletActif.value) {
      case 'regions':
        if (editMode.value) await modifierRegion(id, editId.value, body)
        else await creerRegion(id, body)
        await chargerRegions(id); break
      case 'groupes':
        if (editMode.value) await modifierGroupeEthnique(id, editId.value, body)
        else await creerGroupeEthnique(id, body)
        await chargerGroupesEthniques(id); break
      case 'alliances':
        if (editMode.value) await modifierAlliance(id, editId.value, body)
        else await creerAlliance(id, body)
        await chargerAlliances(id); break
      case 'contes':
        if (editMode.value) await modifierConte(id, editId.value, body)
        else await creerConte(id, body)
        await chargerContes(id, filtreTypeConte.value || undefined); break
      case 'sites':
        if (editMode.value) await modifierSiteTouristique(id, editId.value, body)
        else await creerSiteTouristique(id, body)
        await chargerSitesTouristiques(id); break
      case 'secteurs':
        if (editMode.value) await modifierSecteur(id, editId.value, body)
        else await creerSecteur(id, body)
        await chargerSecteurs(id); break
      case 'saisons':
        if (editMode.value) await modifierSaison(id, editId.value, body)
        else await creerSaison(id, body)
        await chargerSaisons(id); break
      case 'liens':
        if (editMode.value) await modifierLienInterethnique(id, editId.value, body)
        else await creerLienInterethnique(id, body)
        await chargerLiensInterethniques(id); break
    }
    showModal.value = false
    await chargerDetail(id)
  } catch (e: any) {
    modalError.value = e?.data?.error || e?.message || 'Erreur'
  } finally { modalLoading.value = false }
}

const confirmerSuppression = (item: any) => {
  deleteTarget.value = { id: item.id, nom: item.nom || item.titre || 'cet element' }
  showDelete.value = true
}

const executerSuppression = async () => {
  if (!deleteTarget.value) return
  deleteLoading.value = true
  try {
    const tid = deleteTarget.value.id
    switch (ongletActif.value) {
      case 'regions': await supprimerRegion(id, tid); await chargerRegions(id); break
      case 'groupes': await supprimerGroupeEthnique(id, tid); await chargerGroupesEthniques(id); break
      case 'alliances': await supprimerAlliance(id, tid); await chargerAlliances(id); break
      case 'contes': await supprimerConte(id, tid); await chargerContes(id, filtreTypeConte.value || undefined); break
      case 'sites': await supprimerSiteTouristique(id, tid); await chargerSitesTouristiques(id); break
      case 'secteurs': await supprimerSecteur(id, tid); await chargerSecteurs(id); break
      case 'saisons': await supprimerSaison(id, tid); await chargerSaisons(id); break
      case 'liens': await supprimerLienInterethnique(id, tid); await chargerLiensInterethniques(id); break
    }
    showDelete.value = false
    await chargerDetail(id)
  } catch {} finally { deleteLoading.value = false }
}

// ── Stats cards ─────────────────────────────────────────────

const statsCards = computed<StatsCardData[]>(() => {
  if (!ficheDetail.value) return []
  const f = ficheDetail.value
  return [
    { label: 'Regions', value: f.nb_regions, icon: 'location-dot', color: 'bg-primary/10 text-primary' },
    { label: 'Ethnies', value: f.nb_groupes_ethniques, icon: 'users', color: 'bg-info/10 text-info' },
    { label: 'Alliances', value: f.nb_alliances, icon: 'handshake', color: 'bg-success/10 text-success' },
    { label: 'Contes', value: f.nb_contes, icon: 'book-open', color: 'bg-warning/10 text-warning' },
    { label: 'Sites', value: f.nb_sites_touristiques, icon: 'map-marker-alt', color: 'bg-error/10 text-error' },
    { label: 'Secteurs', value: f.nb_secteurs, icon: 'building', color: 'bg-accent/10 text-accent' },
    { label: 'Saisons', value: f.nb_saisons, icon: 'calendar-days', color: 'bg-secondary/10 text-secondary' },
    { label: 'Liens', value: f.nb_liens_interethniques, icon: 'link', color: 'bg-neutral/10 text-neutral' },
  ]
})

// ── Badge « Vérifié » des sites (US3) ───────────────────────

const basculerVerification = async (site: { id: string, verifie: boolean }) => {
  await definirVerificationSite(id, site.id, !site.verifie)
  await chargerSitesTouristiques(id)
}

// ── Chargement par onglet ───────────────────────────────────

const tabDataLoaded = reactive<Record<string, boolean>>({})

const chargerDonneesOnglet = async (tab: TabType) => {
  if (tab === 'general' || tabDataLoaded[tab]) return
  switch (tab) {
    case 'regions': await chargerRegions(id); break
    case 'groupes': await chargerGroupesEthniques(id); break
    case 'alliances': await chargerAlliances(id); break
    case 'contes': await chargerContes(id); break
    case 'sites': await chargerSitesTouristiques(id); break
    case 'secteurs': await chargerSecteurs(id); break
    case 'saisons': await chargerSaisons(id); break
    case 'liens': await chargerLiensInterethniques(id); break
  }
  tabDataLoaded[tab] = true
}

watch(ongletActif, (tab) => chargerDonneesOnglet(tab))

watch(filtreTypeConte, (val) => {
  chargerContes(id, val || undefined)
})

onMounted(async () => {
  await chargerDetail(id)
  remplirForm()
})
</script>

<template>
  <div>
    <AdminPageHeader :titre="ficheDetail?.pays_nom || 'Chargement...'" sous-titre="Modifier la fiche territoire">
      <template #actions>
        <NuxtLink to="/admin/profils-pays" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Retour
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div v-if="loading && !ficheDetail" class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <template v-else-if="ficheDetail">
      <!-- Bannière de modération : fiche bloquée par signalements -->
      <div v-if="ficheDetail.bloquee" class="alert alert-error mb-4 flex-col sm:flex-row items-start sm:items-center">
        <div class="flex items-start gap-3 flex-1">
          <font-awesome-icon icon="ban" class="text-xl mt-0.5" />
          <div>
            <h3 class="font-bold">Territoire bloqué (retiré du public)</h3>
            <div class="text-sm opacity-90">
              Cette fiche a atteint le seuil de signalements ({{ ficheDetail.nombre_signalements }}). Elle n'est plus visible publiquement.
              Débloquez-la pour la rendre de nouveau accessible (les signalements seront purgés).
            </div>
          </div>
        </div>
        <button class="btn btn-sm btn-neutral" :disabled="debloquage" @click="debloquerFiche">
          <span v-if="debloquage" class="loading loading-spinner loading-xs" />
          <font-awesome-icon v-else icon="unlock" class="mr-1" />
          Débloquer
        </button>
      </div>

      <!-- Indicateur de signalements (non bloquée mais signalée) -->
      <div v-else-if="ficheDetail.nombre_signalements > 0" class="alert alert-warning mb-4">
        <font-awesome-icon icon="flag" />
        <span class="text-sm">Ce territoire a reçu {{ ficheDetail.nombre_signalements }} signalement(s) (seuil de blocage : 10).</span>
      </div>

      <!-- Stats cards -->
      <div class="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-8 gap-2 mb-4">
        <AdminStatsCard v-for="(stat, index) in statsCards" :key="index" :stat="stat" />
      </div>

      <!-- Onglets -->
      <div class="tabs tabs-bordered mb-4 flex-wrap">
        <button
          v-for="tab in tabs"
          :key="tab.id"
          :class="['tab', { 'tab-active': ongletActif === tab.id }]"
          @click="ongletActif = tab.id"
        >
          <font-awesome-icon :icon="tab.icon" class="mr-1" />
          <span class="hidden sm:inline">{{ tab.label }}</span>
          <span v-if="tab.countKey" class="badge badge-sm ml-1">{{ getCount(tab.countKey) }}</span>
        </button>
      </div>

      <!-- ══════ Onglet General ══════ -->
      <div v-if="ongletActif === 'general'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <div v-if="erreurLocale || error" class="alert alert-error mb-4">
            <font-awesome-icon icon="circle-exclamation" />
            <span>{{ erreurLocale || error }}</span>
          </div>
          <div v-if="successMsg" class="alert alert-success mb-4">
            <font-awesome-icon icon="circle-check" />
            <span>{{ successMsg }}</span>
          </div>

          <form @submit.prevent="sauvegarderGeneral" class="space-y-6">
            <!-- Infos de base -->
            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Informations generales</h3>
              <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                <div class="form-control">
                  <label class="label"><span class="label-text">Slogan</span></label>
                  <input v-model="form.slogan" type="text" class="input input-bordered">
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Population</span></label>
                  <input v-model.number="form.population" type="number" class="input input-bordered">
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Superficie (km2)</span></label>
                  <input v-model.number="form.superficie_km2" type="number" step="0.01" class="input input-bordered">
                </div>
              </div>
            </div>

            <!-- Description -->
            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Description</h3>
              <div class="form-control">
                <label class="label"><span class="label-text">Biographie</span></label>
                <textarea v-model="form.biographie" class="textarea textarea-bordered h-32" />
              </div>
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div class="form-control">
                  <label class="label"><span class="label-text">Contexte</span></label>
                  <textarea v-model="form.contexte" class="textarea textarea-bordered h-24" />
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Contexte historique</span></label>
                  <textarea v-model="form.contexte_historique" class="textarea textarea-bordered h-24" />
                </div>
              </div>
            </div>

            <!-- Images -->
            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Images</h3>
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div class="form-control">
                  <label class="label"><span class="label-text">Image couverture</span></label>
                  <input v-model="form.image_couverture_url" type="url" class="input input-bordered">
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Drapeau</span></label>
                  <input v-model="form.image_drapeau_url" type="url" class="input input-bordered">
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Embleme</span></label>
                  <input v-model="form.image_embleme_url" type="url" class="input input-bordered">
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Devise</span></label>
                  <input v-model="form.image_devise_url" type="url" class="input input-bordered">
                </div>
              </div>
            </div>

            <!-- Culture -->
            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Culture & symboles</h3>
              <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                <div class="form-control">
                  <label class="label"><span class="label-text">Hymne national</span></label>
                  <input v-model="form.hymne_national" type="text" class="input input-bordered">
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Langue officielle</span></label>
                  <input v-model="form.langue_officielle" type="text" class="input input-bordered">
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Langues populaires</span></label>
                  <input v-model="form.langues_populaires" type="text" class="input input-bordered">
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Monnaie</span></label>
                  <input v-model="form.monnaie" type="text" class="input input-bordered">
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Fuseau horaire</span></label>
                  <input v-model="form.fuseau_horaire" type="text" class="input input-bordered">
                </div>
              </div>
            </div>

            <div class="flex items-center justify-between pt-4">
              <div class="text-sm text-base-content/50">
                <span v-if="ficheDetail.cree_par_nom">Cree par {{ ficheDetail.cree_par_nom }}</span>
                <br>Territoire: {{ ficheDetail.pays_nom }} ({{ ficheDetail.pays_code }})
              </div>
              <button type="submit" class="btn btn-primary" :class="{ loading: saving }" :disabled="saving">
                <font-awesome-icon v-if="!saving" icon="floppy-disk" class="mr-1" /> Enregistrer
              </button>
            </div>
          </form>
        </div>
      </div>

      <!-- ══════ Onglet Regions ══════ -->
      <div v-else-if="ongletActif === 'regions'" class="space-y-4">
        <div class="flex justify-between items-center">
          <h3 class="text-lg font-semibold">Regions ({{ regions.length }})</h3>
          <button class="btn btn-primary btn-sm" @click="ouvrirAjout">
            <font-awesome-icon icon="plus" class="mr-1" /> Ajouter
          </button>
        </div>
        <div class="card bg-base-100 shadow-sm">
          <div class="card-body">
            <div v-if="!regions.length" class="text-center py-8 text-base-content/50">
              <font-awesome-icon icon="inbox" class="text-4xl mb-2" /><p>Aucune region</p>
            </div>
            <div v-else class="overflow-x-auto">
              <table class="table table-zebra">
                <thead><tr><th>Nom</th><th>Chef-lieu</th><th>Population</th><th class="w-24">Actions</th></tr></thead>
                <tbody>
                  <tr v-for="r in regions" :key="r.id">
                    <td>{{ r.nom }}</td>
                    <td>{{ r.chef_lieu || '-' }}</td>
                    <td>{{ r.population?.toLocaleString('fr-FR') || '-' }}</td>
                    <td>
                      <div class="flex gap-1">
                        <button class="btn btn-ghost btn-xs" @click="ouvrirEdition(r)"><font-awesome-icon icon="pen-to-square" /></button>
                        <button class="btn btn-ghost btn-xs text-error" @click="confirmerSuppression(r)"><font-awesome-icon icon="trash" /></button>
                      </div>
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>
      </div>

      <!-- ══════ Onglet Groupes ethniques ══════ -->
      <div v-else-if="ongletActif === 'groupes'" class="space-y-4">
        <div class="flex justify-between items-center">
          <h3 class="text-lg font-semibold">Groupes ethniques ({{ groupesEthniques.length }})</h3>
          <button class="btn btn-primary btn-sm" @click="ouvrirAjout">
            <font-awesome-icon icon="plus" class="mr-1" /> Ajouter
          </button>
        </div>
        <div class="card bg-base-100 shadow-sm">
          <div class="card-body">
            <div v-if="!groupesEthniques.length" class="text-center py-8 text-base-content/50">
              <font-awesome-icon icon="inbox" class="text-4xl mb-2" /><p>Aucun groupe ethnique</p>
            </div>
            <div v-else class="overflow-x-auto">
              <table class="table table-zebra">
                <thead><tr><th>Nom</th><th>Langues</th><th>Population</th><th>Region</th><th class="w-24">Actions</th></tr></thead>
                <tbody>
                  <tr v-for="g in groupesEthniques" :key="g.id">
                    <td>{{ g.nom }}</td>
                    <td>{{ g.langues || '-' }}</td>
                    <td>{{ g.population_estimee || '-' }}</td>
                    <td>{{ g.region_nom || '-' }}</td>
                    <td>
                      <div class="flex gap-1">
                        <button class="btn btn-ghost btn-xs" @click="ouvrirEdition(g)"><font-awesome-icon icon="pen-to-square" /></button>
                        <button class="btn btn-ghost btn-xs text-error" @click="confirmerSuppression(g)"><font-awesome-icon icon="trash" /></button>
                      </div>
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>
      </div>

      <!-- ══════ Onglet Alliances ══════ -->
      <div v-else-if="ongletActif === 'alliances'" class="space-y-4">
        <div class="flex justify-between items-center">
          <h3 class="text-lg font-semibold">Alliances interethniques ({{ alliances.length }})</h3>
          <button class="btn btn-primary btn-sm" @click="ouvrirAjout">
            <font-awesome-icon icon="plus" class="mr-1" /> Ajouter
          </button>
        </div>
        <div class="card bg-base-100 shadow-sm">
          <div class="card-body">
            <div v-if="!alliances.length" class="text-center py-8 text-base-content/50">
              <font-awesome-icon icon="inbox" class="text-4xl mb-2" /><p>Aucune alliance</p>
            </div>
            <div v-else class="overflow-x-auto">
              <table class="table table-zebra">
                <thead><tr><th>Nom</th><th>Groupes impliques</th><th>Signification</th><th class="w-24">Actions</th></tr></thead>
                <tbody>
                  <tr v-for="a in alliances" :key="a.id">
                    <td>{{ a.nom }}</td>
                    <td>{{ a.groupes_impliques || '-' }}</td>
                    <td class="max-w-xs truncate">{{ a.signification || '-' }}</td>
                    <td>
                      <div class="flex gap-1">
                        <button class="btn btn-ghost btn-xs" @click="ouvrirEdition(a)"><font-awesome-icon icon="pen-to-square" /></button>
                        <button class="btn btn-ghost btn-xs text-error" @click="confirmerSuppression(a)"><font-awesome-icon icon="trash" /></button>
                      </div>
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>
      </div>

      <!-- ══════ Onglet Contes ══════ -->
      <div v-else-if="ongletActif === 'contes'" class="space-y-4">
        <div class="flex justify-between items-center">
          <h3 class="text-lg font-semibold">Contes & Histoires ({{ contes.length }})</h3>
          <div class="flex gap-2">
            <select v-model="filtreTypeConte" class="select select-bordered select-sm">
              <option value="">Tous les types</option>
              <option value="conte">Conte</option>
              <option value="histoire_drole">Histoire drole</option>
              <option value="legende">Legende</option>
              <option value="mythe">Mythe</option>
            </select>
            <button class="btn btn-primary btn-sm" @click="ouvrirAjout">
              <font-awesome-icon icon="plus" class="mr-1" /> Ajouter
            </button>
          </div>
        </div>
        <div class="card bg-base-100 shadow-sm">
          <div class="card-body">
            <div v-if="!contes.length" class="text-center py-8 text-base-content/50">
              <font-awesome-icon icon="inbox" class="text-4xl mb-2" /><p>Aucun conte</p>
            </div>
            <div v-else class="overflow-x-auto">
              <table class="table table-zebra">
                <thead><tr><th>Titre</th><th>Type</th><th>Groupe ethnique</th><th class="w-24">Actions</th></tr></thead>
                <tbody>
                  <tr v-for="c in contes" :key="c.id">
                    <td>{{ c.titre }}</td>
                    <td><span v-if="c.type_conte" class="badge badge-sm badge-outline">{{ c.type_conte }}</span><span v-else>-</span></td>
                    <td>{{ c.groupe_ethnique_nom || '-' }}</td>
                    <td>
                      <div class="flex gap-1">
                        <button class="btn btn-ghost btn-xs" @click="ouvrirEdition(c)"><font-awesome-icon icon="pen-to-square" /></button>
                        <button class="btn btn-ghost btn-xs text-error" @click="confirmerSuppression(c)"><font-awesome-icon icon="trash" /></button>
                      </div>
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>
      </div>

      <!-- ══════ Onglet Sites touristiques ══════ -->
      <div v-else-if="ongletActif === 'sites'" class="space-y-4">
        <div class="flex justify-between items-center">
          <h3 class="text-lg font-semibold">Sites touristiques ({{ sitesTouristiques.length }})</h3>
          <button class="btn btn-primary btn-sm" @click="ouvrirAjout">
            <font-awesome-icon icon="plus" class="mr-1" /> Ajouter
          </button>
        </div>
        <div class="card bg-base-100 shadow-sm">
          <div class="card-body">
            <div v-if="!sitesTouristiques.length" class="text-center py-8 text-base-content/50">
              <font-awesome-icon icon="inbox" class="text-4xl mb-2" /><p>Aucun site touristique</p>
            </div>
            <div v-else class="overflow-x-auto">
              <table class="table table-zebra">
                <thead><tr><th>Nom</th><th>Region</th><th>Coordonnees</th><th>Verifie</th><th class="w-24">Actions</th></tr></thead>
                <tbody>
                  <tr v-for="s in sitesTouristiques" :key="s.id">
                    <td>{{ s.nom }}</td>
                    <td>{{ s.region_nom || '-' }}</td>
                    <td>
                      <span v-if="s.latitude && s.longitude" class="text-sm">{{ s.latitude?.toFixed(4) }}, {{ s.longitude?.toFixed(4) }}</span>
                      <span v-else>-</span>
                    </td>
                    <td>
                      <label class="flex items-center gap-2 cursor-pointer">
                        <input
                          type="checkbox"
                          class="toggle toggle-success toggle-sm"
                          :checked="s.verifie"
                          @change="basculerVerification(s)"
                        />
                        <span v-if="s.verifie" class="badge badge-success badge-sm gap-1">
                          <font-awesome-icon icon="circle-check" /> Verifie
                        </span>
                      </label>
                    </td>
                    <td>
                      <div class="flex gap-1">
                        <button class="btn btn-ghost btn-xs" @click="ouvrirEdition(s)"><font-awesome-icon icon="pen-to-square" /></button>
                        <button class="btn btn-ghost btn-xs text-error" @click="confirmerSuppression(s)"><font-awesome-icon icon="trash" /></button>
                      </div>
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>
      </div>

      <!-- ══════ Onglet Secteurs ══════ -->
      <div v-else-if="ongletActif === 'secteurs'" class="space-y-4">
        <div class="flex justify-between items-center">
          <h3 class="text-lg font-semibold">Secteurs de developpement ({{ secteurs.length }})</h3>
          <button class="btn btn-primary btn-sm" @click="ouvrirAjout">
            <font-awesome-icon icon="plus" class="mr-1" /> Ajouter
          </button>
        </div>
        <div class="card bg-base-100 shadow-sm">
          <div class="card-body">
            <div v-if="!secteurs.length" class="text-center py-8 text-base-content/50">
              <font-awesome-icon icon="inbox" class="text-4xl mb-2" /><p>Aucun secteur</p>
            </div>
            <div v-else class="overflow-x-auto">
              <table class="table table-zebra">
                <thead><tr><th>Nom</th><th>Description</th><th class="w-24">Actions</th></tr></thead>
                <tbody>
                  <tr v-for="s in secteurs" :key="s.id">
                    <td>{{ s.nom }}</td>
                    <td class="max-w-xs truncate">{{ s.description || '-' }}</td>
                    <td>
                      <div class="flex gap-1">
                        <button class="btn btn-ghost btn-xs" @click="ouvrirEdition(s)"><font-awesome-icon icon="pen-to-square" /></button>
                        <button class="btn btn-ghost btn-xs text-error" @click="confirmerSuppression(s)"><font-awesome-icon icon="trash" /></button>
                      </div>
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>
      </div>

      <!-- ══════ Onglet Saisons ══════ -->
      <div v-else-if="ongletActif === 'saisons'" class="space-y-4">
        <div class="flex justify-between items-center">
          <h3 class="text-lg font-semibold">Saisons ({{ saisons.length }})</h3>
          <button class="btn btn-primary btn-sm" @click="ouvrirAjout">
            <font-awesome-icon icon="plus" class="mr-1" /> Ajouter
          </button>
        </div>
        <div class="card bg-base-100 shadow-sm">
          <div class="card-body">
            <div v-if="!saisons.length" class="text-center py-8 text-base-content/50">
              <font-awesome-icon icon="inbox" class="text-4xl mb-2" /><p>Aucune saison</p>
            </div>
            <div v-else class="overflow-x-auto">
              <table class="table table-zebra">
                <thead><tr><th>Nom</th><th>Mois debut</th><th>Mois fin</th><th>Description</th><th class="w-24">Actions</th></tr></thead>
                <tbody>
                  <tr v-for="s in saisons" :key="s.id">
                    <td>{{ s.nom }}</td>
                    <td>{{ s.mois_debut ? moisNoms[s.mois_debut - 1] : '-' }}</td>
                    <td>{{ s.mois_fin ? moisNoms[s.mois_fin - 1] : '-' }}</td>
                    <td class="max-w-xs truncate">{{ s.description || '-' }}</td>
                    <td>
                      <div class="flex gap-1">
                        <button class="btn btn-ghost btn-xs" @click="ouvrirEdition(s)"><font-awesome-icon icon="pen-to-square" /></button>
                        <button class="btn btn-ghost btn-xs text-error" @click="confirmerSuppression(s)"><font-awesome-icon icon="trash" /></button>
                      </div>
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>
      </div>

      <!-- ══════ Onglet Liens interethniques ══════ -->
      <div v-else-if="ongletActif === 'liens'" class="space-y-4">
        <div class="flex justify-between items-center">
          <h3 class="text-lg font-semibold">Liens interethniques ({{ liensInterethniques.length }})</h3>
          <button class="btn btn-primary btn-sm" @click="ouvrirAjout">
            <font-awesome-icon icon="plus" class="mr-1" /> Ajouter
          </button>
        </div>
        <div class="card bg-base-100 shadow-sm">
          <div class="card-body">
            <div v-if="!liensInterethniques.length" class="text-center py-8 text-base-content/50">
              <font-awesome-icon icon="inbox" class="text-4xl mb-2" /><p>Aucun lien interethnique</p>
            </div>
            <div v-else class="overflow-x-auto">
              <table class="table table-zebra">
                <thead><tr><th>Territoire lie</th><th>Type</th><th>Description</th><th class="w-24">Actions</th></tr></thead>
                <tbody>
                  <tr v-for="l in liensInterethniques" :key="l.id">
                    <td>{{ l.pays_lie_nom || '-' }}</td>
                    <td><span v-if="l.type_lien" class="badge badge-sm badge-outline">{{ l.type_lien }}</span><span v-else>-</span></td>
                    <td class="max-w-xs truncate">{{ l.description || '-' }}</td>
                    <td>
                      <div class="flex gap-1">
                        <button class="btn btn-ghost btn-xs" @click="ouvrirEdition(l)"><font-awesome-icon icon="pen-to-square" /></button>
                        <button class="btn btn-ghost btn-xs text-error" @click="confirmerSuppression(l)"><font-awesome-icon icon="trash" /></button>
                      </div>
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>
      </div>

      <!-- ══════ Modal ajout/edition sous-entite ══════ -->
      <div v-if="showModal" class="modal modal-open">
        <div class="modal-box max-w-2xl">
          <h3 class="font-bold text-lg mb-4">{{ editMode ? 'Modifier' : 'Ajouter' }} - {{ modalTitres[ongletActif] }}</h3>

          <div v-if="modalError" class="alert alert-error mb-4">
            <font-awesome-icon icon="circle-exclamation" />
            <span>{{ modalError }}</span>
          </div>

          <div class="space-y-4">
            <!-- Region -->
            <template v-if="ongletActif === 'regions'">
              <div class="form-control">
                <label class="label"><span class="label-text">Nom *</span></label>
                <input v-model="formRegion.nom" type="text" class="input input-bordered" required>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Chef-lieu</span></label>
                <input v-model="formRegion.chef_lieu" type="text" class="input input-bordered">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Population</span></label>
                <input v-model.number="formRegion.population" type="number" class="input input-bordered">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Description</span></label>
                <textarea v-model="formRegion.description" class="textarea textarea-bordered h-20" />
              </div>
            </template>

            <!-- Groupe ethnique -->
            <template v-if="ongletActif === 'groupes'">
              <div class="form-control">
                <label class="label"><span class="label-text">Nom *</span></label>
                <input v-model="formGroupe.nom" type="text" class="input input-bordered" required>
              </div>
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div class="form-control">
                  <label class="label"><span class="label-text">Langues</span></label>
                  <input v-model="formGroupe.langues" type="text" class="input input-bordered">
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Population estimee</span></label>
                  <input v-model="formGroupe.population_estimee" type="text" class="input input-bordered">
                </div>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Objets culturels distinctifs</span></label>
                <textarea v-model="formGroupe.objets_culturels_distinctifs" class="textarea textarea-bordered h-20" />
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Region (UUID)</span></label>
                <input v-model="formGroupe.region_id" type="text" class="input input-bordered" placeholder="UUID de la region">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Description</span></label>
                <textarea v-model="formGroupe.description" class="textarea textarea-bordered h-20" />
              </div>
            </template>

            <!-- Alliance -->
            <template v-if="ongletActif === 'alliances'">
              <div class="form-control">
                <label class="label"><span class="label-text">Nom *</span></label>
                <input v-model="formAlliance.nom" type="text" class="input input-bordered" required>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Groupes impliques</span></label>
                <input v-model="formAlliance.groupes_impliques" type="text" class="input input-bordered">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Signification</span></label>
                <textarea v-model="formAlliance.signification" class="textarea textarea-bordered h-20" />
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Description</span></label>
                <textarea v-model="formAlliance.description" class="textarea textarea-bordered h-20" />
              </div>
            </template>

            <!-- Conte -->
            <template v-if="ongletActif === 'contes'">
              <div class="form-control">
                <label class="label"><span class="label-text">Titre *</span></label>
                <input v-model="formConte.titre" type="text" class="input input-bordered" required>
              </div>
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div class="form-control">
                  <label class="label"><span class="label-text">Type</span></label>
                  <select v-model="formConte.type" class="select select-bordered">
                    <option value="">-- Aucun --</option>
                    <option value="conte">Conte</option>
                    <option value="histoire_drole">Histoire drole</option>
                    <option value="legende">Legende</option>
                    <option value="mythe">Mythe</option>
                  </select>
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Groupe ethnique (UUID)</span></label>
                  <input v-model="formConte.groupe_ethnique_id" type="text" class="input input-bordered" placeholder="UUID">
                </div>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Image URL</span></label>
                <input v-model="formConte.image_url" type="url" class="input input-bordered">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Contenu</span></label>
                <textarea v-model="formConte.contenu" class="textarea textarea-bordered h-32" />
              </div>
            </template>

            <!-- Site touristique -->
            <template v-if="ongletActif === 'sites'">
              <div class="form-control">
                <label class="label"><span class="label-text">Nom *</span></label>
                <input v-model="formSite.nom" type="text" class="input input-bordered" required>
              </div>
              <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                <div class="form-control">
                  <label class="label"><span class="label-text">Latitude</span></label>
                  <input v-model.number="formSite.latitude" type="number" step="0.0001" class="input input-bordered">
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Longitude</span></label>
                  <input v-model.number="formSite.longitude" type="number" step="0.0001" class="input input-bordered">
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Region (UUID)</span></label>
                  <input v-model="formSite.region_id" type="text" class="input input-bordered" placeholder="UUID">
                </div>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Image URL</span></label>
                <input v-model="formSite.image_url" type="url" class="input input-bordered">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Description</span></label>
                <textarea v-model="formSite.description" class="textarea textarea-bordered h-20" />
              </div>
            </template>

            <!-- Secteur -->
            <template v-if="ongletActif === 'secteurs'">
              <div class="form-control">
                <label class="label"><span class="label-text">Nom *</span></label>
                <input v-model="formSecteur.nom" type="text" class="input input-bordered" required>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Description</span></label>
                <textarea v-model="formSecteur.description" class="textarea textarea-bordered h-20" />
              </div>
            </template>

            <!-- Saison -->
            <template v-if="ongletActif === 'saisons'">
              <div class="form-control">
                <label class="label"><span class="label-text">Nom *</span></label>
                <input v-model="formSaison.nom" type="text" class="input input-bordered" required>
              </div>
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div class="form-control">
                  <label class="label"><span class="label-text">Mois de debut</span></label>
                  <select v-model.number="formSaison.mois_debut" class="select select-bordered">
                    <option :value="null">-- Aucun --</option>
                    <option v-for="(m, i) in moisNoms" :key="i" :value="i + 1">{{ m }}</option>
                  </select>
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Mois de fin</span></label>
                  <select v-model.number="formSaison.mois_fin" class="select select-bordered">
                    <option :value="null">-- Aucun --</option>
                    <option v-for="(m, i) in moisNoms" :key="i" :value="i + 1">{{ m }}</option>
                  </select>
                </div>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Description</span></label>
                <textarea v-model="formSaison.description" class="textarea textarea-bordered h-20" />
              </div>
            </template>

            <!-- Lien interethnique -->
            <template v-if="ongletActif === 'liens'">
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div class="form-control">
                  <label class="label"><span class="label-text">Territoire lie (UUID)</span></label>
                  <input v-model="formLien.pays_lie_id" type="text" class="input input-bordered" placeholder="UUID du territoire lie">
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Type de lien</span></label>
                  <select v-model="formLien.type_lien" class="select select-bordered">
                    <option value="">-- Aucun --</option>
                    <option value="migration">Migration</option>
                    <option value="parente">Parente</option>
                    <option value="commerce">Commerce</option>
                  </select>
                </div>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Description</span></label>
                <textarea v-model="formLien.description" class="textarea textarea-bordered h-20" />
              </div>
            </template>
          </div>

          <div class="modal-action">
            <button class="btn btn-ghost" @click="showModal = false">Annuler</button>
            <button class="btn btn-primary" :class="{ loading: modalLoading }" :disabled="modalLoading" @click="sauvegarderSousEntite">
              <font-awesome-icon v-if="!modalLoading" :icon="editMode ? 'floppy-disk' : 'plus'" class="mr-1" />
              {{ editMode ? 'Modifier' : 'Ajouter' }}
            </button>
          </div>
        </div>
        <div class="modal-backdrop" @click="showModal = false" />
      </div>

      <!-- ══════ Modal suppression ══════ -->
      <AdminDeleteConfirm
        v-model:visible="showDelete"
        :titre="deleteTarget?.nom"
        :loading="deleteLoading"
        @confirmer="executerSuppression"
      />
    </template>
  </div>
</template>
