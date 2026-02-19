<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const route = useRoute()
const id = route.params.id as string

const {
  badHabitDetail, medias, loading, error,
  chargerDetail, modifier, chargerMedias, ajouterMedia, retirerMedia,
} = useAdminBadHabits()

const ongletActif = ref('infos')
const saving = ref(false)
const erreurLocale = ref<string | null>(null)
const successMsg = ref<string | null>(null)

// Formulaire infos
const form = reactive({
  titre: '',
  categorie_probleme: '',
  categorie_probleme_detail: '',
  gravite: '',
  pays_id: '',
  region: '',
  ville_quartier_zone: '',
  description_generale: '',
  details_problematique: '',
  preuves_temoignages: '',
  solutions_proposees: '',
  publication_anonyme: false,
  geolocalisation_autorisee: false,
  longitude: null as number | null,
  latitude: null as number | null,
  etat: '',
})

// Formulaire ajout media
const mediaForm = reactive({
  media_url: '',
  type_mime: '',
  ordre: null as number | null,
})

const graviteBadgeClass = (gravite: string | null): string => {
  const classes: Record<string, string> = {
    faible: 'badge-success',
    elevee: 'badge-warning',
    critique: 'badge-error',
  }
  return classes[gravite || ''] || 'badge-neutral'
}

const graviteLabel = (gravite: string | null): string => {
  const labels: Record<string, string> = {
    faible: 'Faible',
    elevee: 'Elevee',
    critique: 'Critique',
  }
  return labels[gravite || ''] || gravite || 'Non defini'
}

const categorieLabel = (categorie: string | null): string => {
  const labels: Record<string, string> = {
    corruption: 'Corruption',
    service_public_defaillant: 'Service public defaillant',
    infrastructure_degradee: 'Infrastructure degradee',
    acces_services_limite: 'Acces services limite',
    insalubrite: 'Insalubrite',
    probleme_securite: 'Probleme securite',
    autre: 'Autre',
  }
  return labels[categorie || ''] || categorie || 'Non defini'
}

const charger = async () => {
  await Promise.all([chargerDetail(id), chargerMedias(id)])
  if (badHabitDetail.value) {
    const d = badHabitDetail.value
    form.titre = d.titre
    form.categorie_probleme = d.categorie_probleme || ''
    form.categorie_probleme_detail = d.categorie_probleme_detail || ''
    form.gravite = d.gravite || ''
    form.pays_id = d.pays_id || ''
    form.region = d.region || ''
    form.ville_quartier_zone = d.ville_quartier_zone || ''
    form.description_generale = d.description_generale || ''
    form.details_problematique = d.details_problematique || ''
    form.preuves_temoignages = d.preuves_temoignages || ''
    form.solutions_proposees = d.solutions_proposees || ''
    form.publication_anonyme = d.publication_anonyme
    form.geolocalisation_autorisee = d.geolocalisation_autorisee
    form.longitude = d.longitude
    form.latitude = d.latitude
    form.etat = d.etat
  }
}

const sauvegarderInfos = async () => {
  saving.value = true
  erreurLocale.value = null
  successMsg.value = null
  try {
    const body: any = {
      titre: form.titre.trim(),
      categorie_probleme: form.categorie_probleme,
      gravite: form.gravite,
      description_generale: form.description_generale.trim(),
      details_problematique: form.details_problematique.trim(),
      publication_anonyme: form.publication_anonyme,
      geolocalisation_autorisee: form.geolocalisation_autorisee,
      etat: form.etat,
    }
    if (form.categorie_probleme === 'autre' && form.categorie_probleme_detail.trim()) {
      body.categorie_probleme_detail = form.categorie_probleme_detail.trim()
    }
    if (form.pays_id.trim()) body.pays_id = form.pays_id.trim()
    if (form.region.trim()) body.region = form.region.trim()
    if (form.ville_quartier_zone.trim()) body.ville_quartier_zone = form.ville_quartier_zone.trim()
    if (form.preuves_temoignages.trim()) body.preuves_temoignages = form.preuves_temoignages.trim()
    if (form.solutions_proposees.trim()) body.solutions_proposees = form.solutions_proposees.trim()
    if (form.geolocalisation_autorisee) {
      if (form.longitude !== null) body.longitude = form.longitude
      if (form.latitude !== null) body.latitude = form.latitude
    }
    await modifier(id, body)
    successMsg.value = 'Signalement mis a jour avec succes'
    setTimeout(() => { successMsg.value = null }, 3000)
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de la sauvegarde'
  } finally {
    saving.value = false
  }
}

// Medias (preuves)
const ajouterNouveauMedia = async () => {
  if (!mediaForm.media_url.trim()) return
  try {
    await ajouterMedia(id, {
      media_url: mediaForm.media_url.trim(),
      type_mime: mediaForm.type_mime || undefined,
      ordre: mediaForm.ordre ?? undefined,
    })
    mediaForm.media_url = ''
    mediaForm.type_mime = ''
    mediaForm.ordre = null
    await chargerMedias(id)
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  }
}

const supprimerMedia = async (mediaId: string) => {
  try {
    await retirerMedia(id, mediaId)
    await chargerMedias(id)
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  }
}

onMounted(() => charger())
</script>

<template>
  <div>
    <AdminPageHeader
      :titre="badHabitDetail?.titre || 'Chargement...'"
      sous-titre="Edition du signalement"
    >
      <template #actions>
        <NuxtLink to="/admin/bad-habits" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Retour
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div v-if="loading && !badHabitDetail" class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <template v-else-if="badHabitDetail">
      <!-- Infos rapides -->
      <div class="flex items-center gap-4 mb-6">
        <div class="avatar placeholder">
          <div class="bg-neutral text-neutral-content rounded-full w-16 h-16 flex items-center justify-center">
            <font-awesome-icon icon="triangle-exclamation" class="text-xl" />
          </div>
        </div>
        <div>
          <h2 class="text-lg font-bold">{{ badHabitDetail.titre }}</h2>
          <p class="text-sm text-base-content/60">
            Par {{ badHabitDetail.auteur_prenom }} {{ badHabitDetail.auteur_nom }} &mdash; {{ new Date(badHabitDetail.created_at).toLocaleDateString('fr-FR') }}
          </p>
          <div class="flex gap-2 mt-1">
            <span class="badge badge-sm" :class="graviteBadgeClass(badHabitDetail.gravite)">
              {{ graviteLabel(badHabitDetail.gravite) }}
            </span>
            <span class="badge badge-sm badge-outline">
              {{ categorieLabel(badHabitDetail.categorie_probleme) }}
            </span>
            <span v-if="badHabitDetail.pays_nom" class="badge badge-xs badge-outline">
              {{ badHabitDetail.pays_nom }}
            </span>
            <span class="badge badge-xs badge-outline">
              <font-awesome-icon icon="thumbs-up" class="mr-1" /> {{ badHabitDetail.nombre_soutiens }}
            </span>
          </div>
        </div>
      </div>

      <!-- Alertes -->
      <div v-if="erreurLocale || error" class="alert alert-error mb-4">
        <font-awesome-icon icon="circle-exclamation" />
        <span>{{ erreurLocale || error }}</span>
        <button class="btn btn-ghost btn-xs" @click="erreurLocale = null">
          <font-awesome-icon icon="xmark" />
        </button>
      </div>
      <div v-if="successMsg" class="alert alert-success mb-4">
        <font-awesome-icon icon="circle-check" />
        <span>{{ successMsg }}</span>
      </div>

      <!-- Onglets -->
      <div role="tablist" class="tabs tabs-bordered mb-6">
        <button role="tab" class="tab" :class="{ 'tab-active': ongletActif === 'infos' }" @click="ongletActif = 'infos'">
          <font-awesome-icon icon="pen" class="mr-1" /> Infos
        </button>
        <button role="tab" class="tab" :class="{ 'tab-active': ongletActif === 'preuves' }" @click="ongletActif = 'preuves'">
          <font-awesome-icon icon="images" class="mr-1" /> Preuves ({{ medias.length }})
        </button>
      </div>

      <!-- Onglet Infos -->
      <div v-if="ongletActif === 'infos'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <form @submit.prevent="sauvegarderInfos" class="space-y-4">
            <!-- Titre + categorie -->
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Titre *</span></label>
                <input v-model="form.titre" type="text" class="input input-bordered" required>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Categorie du probleme *</span></label>
                <select v-model="form.categorie_probleme" class="select select-bordered" required>
                  <option value="corruption">Corruption</option>
                  <option value="service_public_defaillant">Service public defaillant</option>
                  <option value="infrastructure_degradee">Infrastructure degradee</option>
                  <option value="acces_services_limite">Acces services limite</option>
                  <option value="insalubrite">Insalubrite</option>
                  <option value="probleme_securite">Probleme securite</option>
                  <option value="autre">Autre</option>
                </select>
              </div>
            </div>

            <!-- Detail categorie (si autre) -->
            <div v-if="form.categorie_probleme === 'autre'" class="form-control">
              <label class="label"><span class="label-text">Preciser la categorie</span></label>
              <input v-model="form.categorie_probleme_detail" type="text" class="input input-bordered" placeholder="Preciser le type de probleme...">
            </div>

            <!-- Gravite + Etat -->
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Gravite</span></label>
                <select v-model="form.gravite" class="select select-bordered">
                  <option value="faible">Faible</option>
                  <option value="elevee">Elevee</option>
                  <option value="critique">Critique</option>
                </select>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Etat</span></label>
                <select v-model="form.etat" class="select select-bordered">
                  <option value="en_attente">En attente</option>
                  <option value="publie">Publie</option>
                  <option value="suspendu">Suspendu</option>
                </select>
              </div>
            </div>

            <!-- Localisation -->
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Pays (UUID)</span></label>
                <input v-model="form.pays_id" type="text" class="input input-bordered" placeholder="ID du pays">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Region</span></label>
                <input v-model="form.region" type="text" class="input input-bordered" placeholder="Ex: Littoral, Dakar...">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Ville / Quartier / Zone</span></label>
                <input v-model="form.ville_quartier_zone" type="text" class="input input-bordered" placeholder="Ex: Douala, Medina...">
              </div>
            </div>

            <!-- Description generale -->
            <div class="form-control">
              <label class="label"><span class="label-text">Description generale *</span></label>
              <textarea v-model="form.description_generale" class="textarea textarea-bordered" rows="4" required />
            </div>

            <!-- Details problematique -->
            <div class="form-control">
              <label class="label"><span class="label-text">Details de la problematique *</span></label>
              <textarea v-model="form.details_problematique" class="textarea textarea-bordered" rows="4" required />
            </div>

            <!-- Preuves et temoignages -->
            <div class="form-control">
              <label class="label"><span class="label-text">Preuves et temoignages</span></label>
              <textarea v-model="form.preuves_temoignages" class="textarea textarea-bordered" rows="3" placeholder="Temoignages, faits verifiables..." />
            </div>

            <!-- Solutions proposees -->
            <div class="form-control">
              <label class="label"><span class="label-text">Solutions proposees</span></label>
              <textarea v-model="form.solutions_proposees" class="textarea textarea-bordered" rows="3" placeholder="Quelles solutions envisagez-vous ?" />
            </div>

            <!-- Options de publication -->
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label cursor-pointer justify-start gap-3">
                  <input v-model="form.publication_anonyme" type="checkbox" class="toggle toggle-primary" />
                  <span class="label-text">Publication anonyme</span>
                </label>
              </div>
              <div class="form-control">
                <label class="label cursor-pointer justify-start gap-3">
                  <input v-model="form.geolocalisation_autorisee" type="checkbox" class="toggle toggle-primary" />
                  <span class="label-text">Geolocalisation autorisee</span>
                </label>
              </div>
            </div>

            <!-- Coordonnees GPS (si geolocalisation autorisee) -->
            <div v-if="form.geolocalisation_autorisee" class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Longitude</span></label>
                <input v-model.number="form.longitude" type="number" step="any" class="input input-bordered" placeholder="Ex: 9.7023">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Latitude</span></label>
                <input v-model.number="form.latitude" type="number" step="any" class="input input-bordered" placeholder="Ex: 4.0511">
              </div>
            </div>

            <div class="flex justify-end pt-2">
              <button type="submit" class="btn btn-primary" :class="{ loading: saving }" :disabled="saving">
                <font-awesome-icon v-if="!saving" icon="floppy-disk" class="mr-1" />
                Enregistrer
              </button>
            </div>
          </form>
        </div>
      </div>

      <!-- Onglet Preuves -->
      <div v-if="ongletActif === 'preuves'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <h3 class="font-semibold mb-4">Preuves visuelles et documents</h3>

          <!-- Formulaire ajout media -->
          <div class="flex gap-2 mb-4">
            <input v-model="mediaForm.media_url" type="text" class="input input-bordered flex-1" placeholder="URL du media (image, document...)">
            <input v-model="mediaForm.type_mime" type="text" class="input input-bordered w-40" placeholder="Type MIME">
            <input v-model.number="mediaForm.ordre" type="number" class="input input-bordered w-20" placeholder="Ordre" min="0">
            <button class="btn btn-primary btn-sm" @click="ajouterNouveauMedia" :disabled="!mediaForm.media_url.trim()">
              <font-awesome-icon icon="plus" class="mr-1" /> Ajouter
            </button>
          </div>

          <!-- Galerie des medias -->
          <div v-if="medias.length" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            <div
              v-for="media in medias"
              :key="media.id"
              class="card card-compact bg-base-200"
            >
              <figure v-if="media.type_mime?.startsWith('image/')" class="h-40">
                <img :src="media.media_url" alt="Preuve" class="w-full h-full object-cover">
              </figure>
              <div v-else class="h-40 flex items-center justify-center bg-base-300">
                <font-awesome-icon icon="file" class="text-4xl text-base-content/30" />
              </div>
              <div class="card-body">
                <p class="text-xs truncate">{{ media.media_url }}</p>
                <p class="text-xs text-base-content/50">
                  {{ media.type_mime || 'Type inconnu' }}
                  <span v-if="media.ordre !== null"> &mdash; Ordre : {{ media.ordre }}</span>
                </p>
                <div class="card-actions justify-end">
                  <button class="btn btn-error btn-xs" @click="supprimerMedia(media.id)">
                    <font-awesome-icon icon="trash" /> Retirer
                  </button>
                </div>
              </div>
            </div>
          </div>
          <div v-else class="text-center text-base-content/50 py-8">
            Aucune preuve associee a ce signalement
          </div>
        </div>
      </div>
    </template>
  </div>
</template>
