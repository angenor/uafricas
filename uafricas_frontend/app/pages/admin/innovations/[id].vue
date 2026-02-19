<script setup lang="ts">
import { STATUTS } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const route = useRoute()
const id = route.params.id as string
const { innovationDetail, chargerDetail, modifier, changerEtat, ajouterMedia, retirerMedia, loading, error } = useAdminInnovations()

const ongletActif = ref('infos')
const saving = ref(false)
const erreurLocale = ref<string | null>(null)
const successMsg = ref<string | null>(null)

const form = reactive({
  titre: '',
  description: '',
  image_couverture_url: '',
  ville: '',
})

const mediaForm = reactive({
  media_url: '',
  type_mime: '',
})

const charger = async () => {
  await chargerDetail(id)
  if (innovationDetail.value) {
    const d = innovationDetail.value
    form.titre = d.titre
    form.description = d.description || ''
    form.image_couverture_url = d.image_couverture_url || ''
    form.ville = d.ville || ''
  }
}

const sauvegarder = async () => {
  saving.value = true
  erreurLocale.value = null
  successMsg.value = null
  try {
    const body: any = {}
    if (form.titre.trim()) body.titre = form.titre.trim()
    if (form.description.trim()) body.description = form.description.trim()
    if (form.image_couverture_url.trim()) body.image_couverture_url = form.image_couverture_url.trim()
    if (form.ville.trim()) body.ville = form.ville.trim()
    await modifier(id, body)
    successMsg.value = 'Innovation mise a jour'
    setTimeout(() => { successMsg.value = null }, 3000)
  }
  catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  }
  finally { saving.value = false }
}

const modererEtat = async (nouvelEtat: string) => {
  saving.value = true
  erreurLocale.value = null
  try {
    await changerEtat(id, nouvelEtat)
    await chargerDetail(id)
    successMsg.value = `Etat change en "${nouvelEtat}"`
    setTimeout(() => { successMsg.value = null }, 3000)
  }
  catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  }
  finally { saving.value = false }
}

const ajouterNouveauMedia = async () => {
  if (!mediaForm.media_url.trim()) return
  try {
    await ajouterMedia(id, {
      media_url: mediaForm.media_url.trim(),
      type_mime: mediaForm.type_mime || undefined,
    })
    mediaForm.media_url = ''
    mediaForm.type_mime = ''
    await chargerDetail(id)
  }
  catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  }
}

const supprimerMedia = async (mediaId: string) => {
  try {
    await retirerMedia(id, mediaId)
    await chargerDetail(id)
  }
  catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  }
}

onMounted(() => charger())
</script>

<template>
  <div>
    <AdminPageHeader :titre="innovationDetail?.titre || 'Chargement...'" sous-titre="Modifier l'innovation">
      <template #actions>
        <NuxtLink to="/admin/innovations" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Retour
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div v-if="loading && !innovationDetail" class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <div v-else-if="innovationDetail">
      <div v-if="erreurLocale || error" class="alert alert-error mb-4">
        <font-awesome-icon icon="circle-exclamation" />
        <span>{{ erreurLocale || error }}</span>
      </div>
      <div v-if="successMsg" class="alert alert-success mb-4">
        <font-awesome-icon icon="circle-check" />
        <span>{{ successMsg }}</span>
      </div>

      <!-- Onglets -->
      <div class="tabs tabs-bordered mb-4">
        <a class="tab" :class="{ 'tab-active': ongletActif === 'infos' }" @click="ongletActif = 'infos'">Infos</a>
        <a class="tab" :class="{ 'tab-active': ongletActif === 'medias' }" @click="ongletActif = 'medias'">
          Medias ({{ innovationDetail.medias?.length || 0 }})
        </a>
      </div>

      <!-- Onglet Infos -->
      <div v-if="ongletActif === 'infos'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <!-- Moderation etat -->
          <div class="flex items-center gap-2 mb-4 p-3 bg-base-200 rounded-lg">
            <span class="font-semibold text-sm">Etat actuel :</span>
            <span
              class="badge"
              :class="{
                'badge-success': innovationDetail.etat === 'publie',
                'badge-info': innovationDetail.etat === 'brouillon',
                'badge-warning': innovationDetail.etat === 'suspendu',
                'badge-neutral': innovationDetail.etat === 'supprime',
              }"
            >
              {{ STATUTS[innovationDetail.etat]?.label || innovationDetail.etat }}
            </span>
            <div class="ml-auto flex gap-1">
              <button
                v-if="innovationDetail.etat !== 'publie'"
                class="btn btn-success btn-xs"
                :disabled="saving"
                @click="modererEtat('publie')"
              >
                Publier
              </button>
              <button
                v-if="innovationDetail.etat !== 'suspendu'"
                class="btn btn-warning btn-xs"
                :disabled="saving"
                @click="modererEtat('suspendu')"
              >
                Suspendre
              </button>
              <button
                v-if="innovationDetail.etat === 'publie'"
                class="btn btn-ghost btn-xs"
                :disabled="saving"
                @click="modererEtat('brouillon')"
              >
                Depublier
              </button>
            </div>
          </div>

          <form @submit.prevent="sauvegarder" class="space-y-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Titre *</span></label>
              <input v-model="form.titre" type="text" class="input input-bordered" required>
            </div>

            <div class="form-control">
              <label class="label"><span class="label-text">Description</span></label>
              <textarea v-model="form.description" class="textarea textarea-bordered" rows="5" />
            </div>

            <div class="form-control">
              <label class="label"><span class="label-text">URL image de couverture</span></label>
              <input v-model="form.image_couverture_url" type="text" class="input input-bordered">
            </div>

            <div class="form-control">
              <label class="label"><span class="label-text">Ville</span></label>
              <input v-model="form.ville" type="text" class="input input-bordered">
            </div>

            <!-- Infos lecture seule -->
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4 pt-2">
              <div class="text-sm">
                <span class="font-semibold">Domaine :</span>
                {{ innovationDetail.domaine_nom || 'Non defini' }}
              </div>
              <div class="text-sm">
                <span class="font-semibold">Organisation :</span>
                {{ innovationDetail.organisation_nom || 'Non definie' }}
              </div>
              <div class="text-sm">
                <span class="font-semibold">Pays :</span>
                {{ innovationDetail.pays_nom || 'Non defini' }}
              </div>
            </div>

            <div class="flex items-center justify-between pt-4">
              <div class="text-sm text-base-content/50">
                Auteur : {{ innovationDetail.auteur_prenom }} {{ innovationDetail.auteur_nom }} |
                Vues : {{ innovationDetail.nombre_vues }}
              </div>
              <button type="submit" class="btn btn-primary" :class="{ loading: saving }" :disabled="saving">
                <font-awesome-icon v-if="!saving" icon="floppy-disk" class="mr-1" /> Enregistrer
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
            <button class="btn btn-primary btn-sm" @click="ajouterNouveauMedia" :disabled="!mediaForm.media_url.trim()">
              <font-awesome-icon icon="plus" class="mr-1" /> Ajouter
            </button>
          </div>

          <!-- Liste medias -->
          <div v-if="innovationDetail.medias?.length" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            <div
              v-for="media in innovationDetail.medias"
              :key="media.id"
              class="card card-compact bg-base-200"
            >
              <figure v-if="media.type_mime?.startsWith('image/')" class="h-40">
                <img :src="media.media_url" :alt="'Media'" class="w-full h-full object-cover">
              </figure>
              <div class="card-body">
                <p class="text-xs truncate">{{ media.media_url }}</p>
                <p class="text-xs text-base-content/50">{{ media.type_mime || 'Type inconnu' }}</p>
                <div class="card-actions justify-end">
                  <button class="btn btn-error btn-xs" @click="supprimerMedia(media.id)">
                    <font-awesome-icon icon="trash" /> Retirer
                  </button>
                </div>
              </div>
            </div>
          </div>
          <div v-else class="text-center text-base-content/50 py-8">
            Aucun media associe a cette innovation
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
