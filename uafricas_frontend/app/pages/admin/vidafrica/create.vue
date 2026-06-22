<script setup lang="ts">
import { PAYS_AFRICAINS } from '~/composables/useEvenements'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const router = useRouter()
const { creer, loading, error } = useAdminVidafrica()

const form = reactive({
  titre: '',
  description: '',
  auteur_reel: '',
})

const territoires = ref<string[]>([])
const territoireAAjouter = ref('')
const dechargeDroits = ref(false)
const fichierVideo = ref<File | null>(null)
const vignette = ref<File | null>(null)
const erreurLocale = ref('')
const soumettreLoading = ref(false)

const territoiresDisponibles = computed(() =>
  PAYS_AFRICAINS.filter(p => !territoires.value.includes(p)),
)

const ajouterTerritoire = () => {
  const t = territoireAAjouter.value
  if (t && !territoires.value.includes(t)) territoires.value.push(t)
  territoireAAjouter.value = ''
}

const retirerTerritoire = (t: string) => {
  territoires.value = territoires.value.filter(x => x !== t)
}

const onFichierVideoChange = (e: Event) => {
  const input = e.target as HTMLInputElement
  if (input.files?.[0]) {
    const file = input.files[0]
    const ext = file.name.split('.').pop()?.toLowerCase()
    if (!['mp4', 'webm'].includes(ext || '')) {
      erreurLocale.value = 'Format vidéo invalide. Formats acceptés : MP4, WebM'
      fichierVideo.value = null
      return
    }
    if (file.size > 500 * 1024 * 1024) {
      erreurLocale.value = 'Le fichier vidéo dépasse la limite de 500 Mo'
      fichierVideo.value = null
      return
    }
    erreurLocale.value = ''
    fichierVideo.value = file
  }
}

const onVignetteChange = (e: Event) => {
  const input = e.target as HTMLInputElement
  if (input.files?.[0]) {
    const file = input.files[0]
    const ext = file.name.split('.').pop()?.toLowerCase()
    if (!['jpg', 'jpeg', 'png', 'webp'].includes(ext || '')) {
      erreurLocale.value = 'Format vignette invalide. Formats acceptés : JPG, PNG, WebP'
      vignette.value = null
      return
    }
    if (file.size > 5 * 1024 * 1024) {
      erreurLocale.value = 'La vignette dépasse la limite de 5 Mo'
      vignette.value = null
      return
    }
    erreurLocale.value = ''
    vignette.value = file
  }
}

const soumettre = async () => {
  erreurLocale.value = ''

  if (!form.titre.trim()) {
    erreurLocale.value = 'Le titre est requis'
    return
  }
  if (!fichierVideo.value) {
    erreurLocale.value = 'Le fichier vidéo est requis'
    return
  }

  soumettreLoading.value = true
  try {
    const formData = new FormData()
    formData.append('titre', form.titre.trim())
    if (form.description.trim()) {
      formData.append('description', form.description.trim())
    }
    territoires.value.forEach(t => formData.append('territoires', t))
    if (form.auteur_reel.trim()) {
      formData.append('auteur_reel', form.auteur_reel.trim())
    }
    formData.append('decharge_droits', dechargeDroits.value ? 'true' : 'false')
    formData.append('fichier_video', fichierVideo.value)
    if (vignette.value) {
      formData.append('vignette', vignette.value)
    }

    const result = await creer(formData)
    if (result?.id) {
      router.push(`/admin/vidafrica/${result.id}`)
    }
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de la création'
  } finally {
    soumettreLoading.value = false
  }
}
</script>

<template>
  <div>
    <AdminPageHeader titre="Nouvelle vidéo" sous-titre="Uploader une vidéo Vidafrica">
      <template #actions>
        <NuxtLink to="/admin/vidafrica" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Retour
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div class="card bg-base-100 shadow-sm">
      <div class="card-body">
        <div v-if="erreurLocale || error" class="alert alert-error mb-4">
          <font-awesome-icon icon="circle-exclamation" />
          <span>{{ erreurLocale || error }}</span>
        </div>

        <div class="form-control mb-4">
          <label class="label"><span class="label-text font-medium">Titre *</span></label>
          <input
            v-model="form.titre"
            type="text"
            class="input input-bordered"
            placeholder="Titre de la vidéo"
            maxlength="300"
          />
        </div>

        <div class="form-control mb-4">
          <label class="label"><span class="label-text font-medium">Description</span></label>
          <textarea
            v-model="form.description"
            class="textarea textarea-bordered h-24"
            placeholder="Description de la vidéo (optionnel)"
          />
        </div>

        <div class="form-control mb-4">
          <label class="label"><span class="label-text font-medium">Territoires</span></label>
          <select
            v-model="territoireAAjouter"
            class="select select-bordered"
            @change="ajouterTerritoire"
          >
            <option value="">Ajouter un territoire…</option>
            <option v-for="p in territoiresDisponibles" :key="p" :value="p">{{ p }}</option>
          </select>
          <div v-if="territoires.length" class="flex flex-wrap gap-2 mt-2">
            <span v-for="t in territoires" :key="t" class="badge badge-primary gap-1">
              {{ t }}
              <button type="button" class="hover:text-error" @click="retirerTerritoire(t)">
                <font-awesome-icon icon="xmark" />
              </button>
            </span>
          </div>
        </div>

        <div class="form-control mb-4">
          <label class="label"><span class="label-text font-medium">Auteur réel</span></label>
          <input
            v-model="form.auteur_reel"
            type="text"
            class="input input-bordered"
            placeholder="Nom de l'auteur réel de la chanson (le cas échéant)"
            maxlength="300"
          />
        </div>

        <div class="form-control mb-4">
          <label class="label cursor-pointer justify-start gap-3">
            <input v-model="dechargeDroits" type="checkbox" class="checkbox checkbox-primary" />
            <span class="label-text">Le contributeur ne revendique aucun droit sur cette œuvre (décharge de droits)</span>
          </label>
        </div>

        <div class="divider">Fichiers</div>

        <div class="form-control mb-4">
          <label class="label"><span class="label-text font-medium">Fichier vidéo * (MP4, WebM — max 500 Mo)</span></label>
          <input
            type="file"
            class="file-input file-input-bordered"
            accept=".mp4,.webm"
            @change="onFichierVideoChange"
          />
          <label v-if="fichierVideo" class="label">
            <span class="label-text-alt text-success">{{ fichierVideo.name }} ({{ (fichierVideo.size / 1024 / 1024).toFixed(1) }} Mo)</span>
          </label>
        </div>

        <div class="form-control mb-4">
          <label class="label"><span class="label-text font-medium">Vignette (JPG, PNG, WebP — max 5 Mo)</span></label>
          <input
            type="file"
            class="file-input file-input-bordered"
            accept=".jpg,.jpeg,.png,.webp"
            @change="onVignetteChange"
          />
          <label v-if="vignette" class="label">
            <span class="label-text-alt text-success">{{ vignette.name }}</span>
          </label>
        </div>

        <div class="card-actions justify-end mt-4">
          <NuxtLink to="/admin/vidafrica" class="btn btn-ghost">Annuler</NuxtLink>
          <button
            class="btn btn-primary"
            :class="{ loading: soumettreLoading }"
            :disabled="soumettreLoading"
            @click="soumettre"
          >
            <font-awesome-icon icon="upload" class="mr-1" /> Créer la vidéo
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
