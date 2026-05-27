<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const route = useRoute()
const id = route.params.id as string

const {
  ideaForceDetail, medias, loading, error,
  chargerDetail, modifier,
  chargerMedias, ajouterMedia, retirerMedia,
} = useAdminIdeaForces()

const ongletActif = ref('infos')
const saving = ref(false)
const erreurLocale = ref<string | null>(null)
const successMsg = ref<string | null>(null)

// Formulaire infos
const form = reactive({
  titre: '',
  categorie_proposition: '',
  categorie_proposition_detail: '',
  urgence: '',
  pays_id: '',
  region: '',
  ville_quartier_zone: '',
  description_generale: '',
  details_proposition: '',
  plan_implementation: '',
  ressources_necessaires: '',
  impact_attendu: '',
  etat: '',
})

// Formulaire media
const mediaForm = reactive({
  media_url: '',
  type_mime: '',
  ordre: 0,
})

const categorieLabel = (cat: string | null): string => {
  const labels: Record<string, string> = {
    amelioration_gouvernance: 'Gouvernance',
    education_formation: 'Education',
    sante_publique: 'Sante',
    emploi_jeunes: 'Emploi',
    environnement: 'Environnement',
    transport: 'Transport',
    autre: 'Autre',
  }
  return labels[cat || ''] || cat || 'Non defini'
}

const categorieBadgeClass = (cat: string | null): string => {
  const classes: Record<string, string> = {
    amelioration_gouvernance: 'badge-info',
    education_formation: 'badge-primary',
    sante_publique: 'badge-success',
    emploi_jeunes: 'badge-warning',
    environnement: 'badge-accent',
    transport: 'badge-secondary',
    autre: 'badge-neutral',
  }
  return classes[cat || ''] || 'badge-neutral'
}

const urgenceBadgeClass = (urgence: string | null): string => {
  const classes: Record<string, string> = {
    faible: 'badge-success',
    elevee: 'badge-warning',
    critique: 'badge-error',
  }
  return classes[urgence || ''] || 'badge-neutral'
}

const urgenceLabel = (urgence: string | null): string => {
  const labels: Record<string, string> = {
    faible: 'Faible',
    elevee: 'Elevee',
    critique: 'Critique',
  }
  return labels[urgence || ''] || urgence || 'Non defini'
}

const charger = async () => {
  await Promise.all([chargerDetail(id), chargerMedias(id)])
  if (ideaForceDetail.value) {
    const d = ideaForceDetail.value
    form.titre = d.titre || ''
    form.categorie_proposition = d.categorie_proposition || ''
    form.categorie_proposition_detail = d.categorie_proposition_detail || ''
    form.urgence = d.urgence || ''
    form.pays_id = d.pays_id || ''
    form.region = d.region || ''
    form.ville_quartier_zone = d.ville_quartier_zone || ''
    form.description_generale = d.description_generale || ''
    form.details_proposition = d.details_proposition || ''
    form.plan_implementation = d.plan_implementation || ''
    form.ressources_necessaires = d.ressources_necessaires || ''
    form.impact_attendu = d.impact_attendu || ''
    form.etat = d.etat || ''
  }
}

const sauvegarder = async () => {
  saving.value = true
  erreurLocale.value = null
  successMsg.value = null
  try {
    const body: any = {
      titre: form.titre.trim(),
      categorie_proposition: form.categorie_proposition,
      urgence: form.urgence,
      etat: form.etat,
    }
    if (form.categorie_proposition === 'autre' && form.categorie_proposition_detail.trim()) {
      body.categorie_proposition_detail = form.categorie_proposition_detail.trim()
    }
    if (form.pays_id.trim()) body.pays_id = form.pays_id.trim()
    if (form.region.trim()) body.region = form.region.trim()
    if (form.ville_quartier_zone.trim()) body.ville_quartier_zone = form.ville_quartier_zone.trim()
    if (form.description_generale.trim()) body.description_generale = form.description_generale.trim()
    if (form.details_proposition.trim()) body.details_proposition = form.details_proposition.trim()
    if (form.plan_implementation.trim()) body.plan_implementation = form.plan_implementation.trim()
    if (form.ressources_necessaires.trim()) body.ressources_necessaires = form.ressources_necessaires.trim()
    if (form.impact_attendu.trim()) body.impact_attendu = form.impact_attendu.trim()
    await modifier(id, body)
    successMsg.value = 'Idee force mise a jour avec succes'
    setTimeout(() => { successMsg.value = null }, 3000)
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de la sauvegarde'
  } finally {
    saving.value = false
  }
}

// Medias
const ajouterNouveauMedia = async () => {
  if (!mediaForm.media_url.trim()) return
  try {
    await ajouterMedia(id, {
      media_url: mediaForm.media_url.trim(),
      type_mime: mediaForm.type_mime || undefined,
      ordre: mediaForm.ordre || undefined,
    })
    mediaForm.media_url = ''
    mediaForm.type_mime = ''
    mediaForm.ordre = 0
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
    <AdminPageHeader :titre="ideaForceDetail?.titre || 'Chargement...'" sous-titre="Edition de l'idee force">
      <template #actions>
        <NuxtLink to="/admin/idea-forces" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Retour
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div v-if="loading && !ideaForceDetail" class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <template v-else-if="ideaForceDetail">
      <!-- Infos rapides -->
      <div class="flex items-center gap-4 mb-6">
        <div class="avatar placeholder">
          <div class="bg-neutral text-neutral-content rounded-full w-16 h-16 flex items-center justify-center">
            <font-awesome-icon icon="lightbulb" class="text-xl" />
          </div>
        </div>
        <div>
          <h2 class="text-lg font-bold">{{ ideaForceDetail.titre }}</h2>
          <p class="text-sm text-base-content/60">
            Par {{ ideaForceDetail.auteur_prenom }} {{ ideaForceDetail.auteur_nom }} &mdash; {{ new Date(ideaForceDetail.created_at).toLocaleDateString('fr-FR') }}
          </p>
          <div class="flex gap-2 mt-1">
            <span class="badge badge-sm" :class="urgenceBadgeClass(ideaForceDetail.urgence)">
              {{ urgenceLabel(ideaForceDetail.urgence) }}
            </span>
            <span class="badge badge-sm" :class="categorieBadgeClass(ideaForceDetail.categorie_proposition)">
              {{ categorieLabel(ideaForceDetail.categorie_proposition) }}
            </span>
            <span v-if="ideaForceDetail.pays_nom" class="badge badge-xs badge-outline">{{ ideaForceDetail.pays_nom }}</span>
            <span class="badge badge-xs badge-outline">
              <font-awesome-icon icon="heart" class="mr-1" /> {{ ideaForceDetail.nombre_soutiens || 0 }}
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
        <button role="tab" class="tab" :class="{ 'tab-active': ongletActif === 'medias' }" @click="ongletActif = 'medias'">
          <font-awesome-icon icon="images" class="mr-1" /> Medias ({{ medias.length }})
        </button>
      </div>

      <!-- Onglet Infos -->
      <div v-if="ongletActif === 'infos'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <form @submit.prevent="sauvegarder" class="space-y-4">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Titre *</span></label>
                <input v-model="form.titre" type="text" class="input input-bordered" required>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Categorie *</span></label>
                <select v-model="form.categorie_proposition" class="select select-bordered" required>
                  <option value="amelioration_gouvernance">Gouvernance</option>
                  <option value="education_formation">Education</option>
                  <option value="sante_publique">Sante</option>
                  <option value="emploi_jeunes">Emploi</option>
                  <option value="environnement">Environnement</option>
                  <option value="transport">Transport</option>
                  <option value="autre">Autre</option>
                </select>
              </div>
            </div>

            <div v-if="form.categorie_proposition === 'autre'" class="form-control">
              <label class="label"><span class="label-text">Precision de la categorie</span></label>
              <input v-model="form.categorie_proposition_detail" type="text" class="input input-bordered" placeholder="Precisez la categorie...">
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Urgence</span></label>
                <select v-model="form.urgence" class="select select-bordered">
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

            <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Territoire (UUID)</span></label>
                <input v-model="form.pays_id" type="text" class="input input-bordered" placeholder="ID du territoire">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Region</span></label>
                <input v-model="form.region" type="text" class="input input-bordered" placeholder="Ex: Littoral, Dakar...">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Ville / Quartier / Zone</span></label>
                <input v-model="form.ville_quartier_zone" type="text" class="input input-bordered" placeholder="Ex: Douala, Bonaberi...">
              </div>
            </div>

            <div class="form-control">
              <label class="label"><span class="label-text">Description generale *</span></label>
              <textarea v-model="form.description_generale" class="textarea textarea-bordered" rows="4" required />
            </div>

            <div class="form-control">
              <label class="label"><span class="label-text">Details de la proposition *</span></label>
              <textarea v-model="form.details_proposition" class="textarea textarea-bordered" rows="4" required />
            </div>

            <div class="form-control">
              <label class="label"><span class="label-text">Plan d'implementation</span></label>
              <textarea v-model="form.plan_implementation" class="textarea textarea-bordered" rows="3" placeholder="Etapes de mise en oeuvre..." />
            </div>

            <div class="form-control">
              <label class="label"><span class="label-text">Ressources necessaires</span></label>
              <textarea v-model="form.ressources_necessaires" class="textarea textarea-bordered" rows="3" placeholder="Budget, materiels, ressources humaines..." />
            </div>

            <div class="form-control">
              <label class="label"><span class="label-text">Impact attendu</span></label>
              <textarea v-model="form.impact_attendu" class="textarea textarea-bordered" rows="3" placeholder="Impact social, economique, environnemental attendu..." />
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

      <!-- Onglet Medias -->
      <div v-if="ongletActif === 'medias'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <!-- Ajouter media -->
          <div class="flex gap-2 mb-4">
            <input v-model="mediaForm.media_url" type="text" class="input input-bordered flex-1" placeholder="URL du media">
            <input v-model="mediaForm.type_mime" type="text" class="input input-bordered w-40" placeholder="Type MIME">
            <input v-model.number="mediaForm.ordre" type="number" class="input input-bordered w-24" placeholder="Ordre" min="0">
            <button class="btn btn-primary btn-sm" @click="ajouterNouveauMedia" :disabled="!mediaForm.media_url.trim()">
              <font-awesome-icon icon="plus" class="mr-1" /> Ajouter
            </button>
          </div>

          <!-- Liste medias -->
          <div v-if="medias.length" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            <div
              v-for="media in medias"
              :key="media.id"
              class="card card-compact bg-base-200"
            >
              <figure v-if="media.type_mime?.startsWith('image/')" class="h-40">
                <img :src="media.media_url" :alt="'Media'" class="w-full h-full object-cover">
              </figure>
              <div v-else class="h-40 flex items-center justify-center bg-base-300">
                <font-awesome-icon icon="file" class="text-4xl text-base-content/30" />
              </div>
              <div class="card-body">
                <p class="text-xs truncate">{{ media.media_url }}</p>
                <p class="text-xs text-base-content/50">
                  {{ media.type_mime || 'Type inconnu' }}
                  <span v-if="media.ordre != null"> | Ordre: {{ media.ordre }}</span>
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
            Aucun media associe a cette idee force
          </div>
        </div>
      </div>
    </template>
  </div>
</template>
