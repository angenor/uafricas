<script setup lang="ts">
import { STATUTS } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const route = useRoute()
const id = route.params.id as string
const { africantiveDetail, chargerDetail, modifier, changerEtat, loading, error } = useAdminAfricantives()

const saving = ref(false)
const erreurLocale = ref<string | null>(null)
const successMsg = ref<string | null>(null)

const form = reactive({
  titre: '',
  description: '',
  image_couverture_url: '',
  ville: '',
})

const charger = async () => {
  await chargerDetail(id)
  if (africantiveDetail.value) {
    const d = africantiveDetail.value
    form.titre = d.titre
    form.description = d.description
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
    successMsg.value = 'Africantive mise a jour'
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

onMounted(() => charger())
</script>

<template>
  <div>
    <AdminPageHeader :titre="africantiveDetail?.titre || 'Chargement...'" sous-titre="Modifier l'africantive">
      <template #actions>
        <NuxtLink to="/admin/africantives" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Retour
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div v-if="loading && !africantiveDetail" class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <div v-else-if="africantiveDetail" class="card bg-base-100 shadow-sm">
      <div class="card-body">
        <div v-if="erreurLocale || error" class="alert alert-error mb-4">
          <font-awesome-icon icon="circle-exclamation" />
          <span>{{ erreurLocale || error }}</span>
        </div>
        <div v-if="successMsg" class="alert alert-success mb-4">
          <font-awesome-icon icon="circle-check" />
          <span>{{ successMsg }}</span>
        </div>

        <!-- Moderation etat -->
        <div class="flex items-center gap-2 mb-4 p-3 bg-base-200 rounded-lg">
          <span class="font-semibold text-sm">État actuel :</span>
          <span
            class="badge"
            :class="{
              'badge-success': africantiveDetail.etat === 'publie',
              'badge-info': africantiveDetail.etat === 'brouillon',
              'badge-warning': africantiveDetail.etat === 'suspendu',
              'badge-neutral': africantiveDetail.etat === 'supprime',
            }"
          >
            {{ STATUTS[africantiveDetail.etat]?.label || africantiveDetail.etat }}
          </span>
          <div class="ml-auto flex gap-1">
            <button
              v-if="africantiveDetail.etat !== 'publie'"
              class="btn btn-success btn-xs"
              :disabled="saving"
              @click="modererEtat('publie')"
            >
              Publier
            </button>
            <button
              v-if="africantiveDetail.etat !== 'suspendu'"
              class="btn btn-warning btn-xs"
              :disabled="saving"
              @click="modererEtat('suspendu')"
            >
              Suspendre
            </button>
            <button
              v-if="africantiveDetail.etat === 'publie'"
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
            <label class="label"><span class="label-text">Description *</span></label>
            <textarea v-model="form.description" class="textarea textarea-bordered" rows="5" required />
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
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4 pt-2">
            <div class="text-sm">
              <span class="font-semibold">Domaine :</span>
              {{ africantiveDetail.domaine_nom || 'Non defini' }}
            </div>
            <div class="text-sm">
              <span class="font-semibold">Territoire :</span>
              {{ africantiveDetail.pays_nom || 'Non defini' }}
            </div>
          </div>

          <div class="flex items-center justify-between pt-4">
            <div class="text-sm text-base-content/50">
              Auteur : {{ africantiveDetail.auteur_prenom }} {{ africantiveDetail.auteur_nom }}
            </div>
            <button type="submit" class="btn btn-primary" :class="{ loading: saving }" :disabled="saving">
              <font-awesome-icon v-if="!saving" icon="floppy-disk" class="mr-1" /> Enregistrer
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>
