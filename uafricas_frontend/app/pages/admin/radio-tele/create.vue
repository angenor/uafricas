<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const route = useRoute()
const router = useRouter()
const type = computed(() => (route.query.type as string) || 'stations')

const { creerStation, creerChaine, creerProgramme, loading, error } = useAdminRadioTele()

const erreurLocale = ref<string | null>(null)

// Station Radio form
const stationForm = reactive({
  nom: '',
  description: '',
  stream_url: '',
  image_couverture_url: '',
  genre: '',
  genres_liste: [] as string[],
  pays_id: '',
  ville: '',
  type_station: 'nationale',
})

const genreInput = ref('')

const ajouterGenre = () => {
  const g = genreInput.value.trim()
  if (g && !stationForm.genres_liste.includes(g)) {
    stationForm.genres_liste.push(g)
    genreInput.value = ''
  }
}

const retirerGenre = (index: number) => {
  stationForm.genres_liste.splice(index, 1)
}

// Chaine TV form
const chaineForm = reactive({
  nom: '',
  description: '',
  stream_url: '',
  image_couverture_url: '',
  categorie: 'generaliste',
  pays_id: '',
  langue: '',
  est_en_direct: false,
})

// Programme Media form
const programmeForm = reactive({
  nom_emission: '',
  type_programme: 'radio',
  description: '',
  image_couverture_url: '',
  video_url: '',
  info_animateur: '',
  info_producteur: '',
  pays_id: '',
  est_international: false,
  langue: '',
  categorie_radio: '',
})

const titrePageMap: Record<string, string> = {
  stations: 'Nouvelle station radio',
  chaines: 'Nouvelle chaine TV',
  programmes: 'Nouveau programme media',
}

const sousTitreMap: Record<string, string> = {
  stations: 'Creer une station de radio',
  chaines: 'Creer une chaine de television',
  programmes: 'Creer un programme media',
}

const soumettre = async () => {
  erreurLocale.value = null

  try {
    if (type.value === 'stations') {
      if (!stationForm.nom.trim()) { erreurLocale.value = 'Le nom de la station est requis'; return }
      if (!stationForm.stream_url.trim()) { erreurLocale.value = "L'URL de stream est requise"; return }

      const body: any = {
        nom: stationForm.nom.trim(),
        stream_url: stationForm.stream_url.trim(),
        type_station: stationForm.type_station,
      }
      if (stationForm.description.trim()) body.description = stationForm.description.trim()
      if (stationForm.image_couverture_url.trim()) body.image_couverture_url = stationForm.image_couverture_url.trim()
      if (stationForm.genre.trim()) body.genre = stationForm.genre.trim()
      if (stationForm.genres_liste.length) body.genres_liste = stationForm.genres_liste
      if (stationForm.pays_id) body.pays_id = stationForm.pays_id
      if (stationForm.ville.trim()) body.ville = stationForm.ville.trim()

      await creerStation(body)
    }
    else if (type.value === 'chaines') {
      if (!chaineForm.nom.trim()) { erreurLocale.value = 'Le nom de la chaine est requis'; return }
      if (!chaineForm.stream_url.trim()) { erreurLocale.value = "L'URL de stream est requise"; return }

      const body: any = {
        nom: chaineForm.nom.trim(),
        stream_url: chaineForm.stream_url.trim(),
        categorie: chaineForm.categorie,
        est_en_direct: chaineForm.est_en_direct,
      }
      if (chaineForm.description.trim()) body.description = chaineForm.description.trim()
      if (chaineForm.image_couverture_url.trim()) body.image_couverture_url = chaineForm.image_couverture_url.trim()
      if (chaineForm.pays_id) body.pays_id = chaineForm.pays_id
      if (chaineForm.langue.trim()) body.langue = chaineForm.langue.trim()

      await creerChaine(body)
    }
    else if (type.value === 'programmes') {
      if (!programmeForm.nom_emission.trim()) { erreurLocale.value = "Le nom de l'emission est requis"; return }

      const body: any = {
        nom_emission: programmeForm.nom_emission.trim(),
        type_programme: programmeForm.type_programme,
        est_international: programmeForm.est_international,
      }
      if (programmeForm.description.trim()) body.description = programmeForm.description.trim()
      if (programmeForm.image_couverture_url.trim()) body.image_couverture_url = programmeForm.image_couverture_url.trim()
      if (programmeForm.video_url.trim()) body.video_url = programmeForm.video_url.trim()
      if (programmeForm.info_animateur.trim()) body.info_animateur = programmeForm.info_animateur.trim()
      if (programmeForm.info_producteur.trim()) body.info_producteur = programmeForm.info_producteur.trim()
      if (programmeForm.pays_id) body.pays_id = programmeForm.pays_id
      if (programmeForm.langue.trim()) body.langue = programmeForm.langue.trim()
      if (programmeForm.categorie_radio) body.categorie_radio = programmeForm.categorie_radio

      await creerProgramme(body)
    }

    router.push('/admin/radio-tele')
  }
  catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de la creation'
  }
}
</script>

<template>
  <div>
    <AdminPageHeader :titre="titrePageMap[type] || 'Nouveau'" :sous-titre="sousTitreMap[type] || ''">
      <template #actions>
        <NuxtLink to="/admin/radio-tele" class="btn btn-ghost btn-sm">
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

        <!-- Formulaire Station Radio -->
        <form v-if="type === 'stations'" @submit.prevent="soumettre" class="space-y-6">
          <div class="space-y-4">
            <h3 class="text-lg font-semibold border-b pb-2">Informations de base</h3>
            <div class="form-control">
              <label class="label"><span class="label-text">Nom de la station *</span></label>
              <input v-model="stationForm.nom" type="text" class="input input-bordered" required placeholder="Ex: Radio Africa 1">
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">URL de stream *</span></label>
              <input v-model="stationForm.stream_url" type="text" class="input input-bordered" required placeholder="https://stream.example.com/live">
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Description</span></label>
              <textarea v-model="stationForm.description" class="textarea textarea-bordered h-32" placeholder="Description de la station..." />
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">URL image de couverture</span></label>
              <input v-model="stationForm.image_couverture_url" type="text" class="input input-bordered" placeholder="https://...">
            </div>
          </div>

          <div class="space-y-4">
            <h3 class="text-lg font-semibold border-b pb-2">Classification</h3>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Genre principal</span></label>
                <input v-model="stationForm.genre" type="text" class="input input-bordered" placeholder="Ex: Musique, Actualites">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Type de station</span></label>
                <select v-model="stationForm.type_station" class="select select-bordered">
                  <option value="nationale">Nationale</option>
                  <option value="locale">Locale</option>
                  <option value="internationale">Internationale</option>
                </select>
              </div>
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Genres (tags)</span></label>
              <div class="flex gap-2">
                <input v-model="genreInput" type="text" class="input input-bordered flex-1" placeholder="Ex: Afrobeat" @keydown.enter.prevent="ajouterGenre">
                <button type="button" class="btn btn-outline" @click="ajouterGenre">Ajouter</button>
              </div>
              <div v-if="stationForm.genres_liste.length" class="flex flex-wrap gap-2 mt-2">
                <span v-for="(g, i) in stationForm.genres_liste" :key="i" class="badge badge-primary gap-1">
                  {{ g }}
                  <button type="button" class="btn btn-ghost btn-xs p-0" @click="retirerGenre(i)">&times;</button>
                </span>
              </div>
            </div>
          </div>

          <div class="space-y-4">
            <h3 class="text-lg font-semibold border-b pb-2">Localisation</h3>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Territoire</span></label>
                <input v-model="stationForm.pays_id" type="text" class="input input-bordered" placeholder="UUID du territoire">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Ville</span></label>
                <input v-model="stationForm.ville" type="text" class="input input-bordered" placeholder="Ex: Dakar">
              </div>
            </div>
          </div>

          <div class="flex justify-end gap-2 pt-4">
            <NuxtLink to="/admin/radio-tele" class="btn btn-ghost">Annuler</NuxtLink>
            <button type="submit" class="btn btn-primary" :class="{ loading }" :disabled="loading">
              <font-awesome-icon v-if="!loading" icon="plus" class="mr-1" /> Creer
            </button>
          </div>
        </form>

        <!-- Formulaire Chaine TV -->
        <form v-else-if="type === 'chaines'" @submit.prevent="soumettre" class="space-y-6">
          <div class="space-y-4">
            <h3 class="text-lg font-semibold border-b pb-2">Informations de base</h3>
            <div class="form-control">
              <label class="label"><span class="label-text">Nom de la chaine *</span></label>
              <input v-model="chaineForm.nom" type="text" class="input input-bordered" required placeholder="Ex: Africa 24">
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">URL de stream *</span></label>
              <input v-model="chaineForm.stream_url" type="text" class="input input-bordered" required placeholder="https://stream.example.com/live">
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Description</span></label>
              <textarea v-model="chaineForm.description" class="textarea textarea-bordered h-32" placeholder="Description de la chaine..." />
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">URL image de couverture</span></label>
              <input v-model="chaineForm.image_couverture_url" type="text" class="input input-bordered" placeholder="https://...">
            </div>
          </div>

          <div class="space-y-4">
            <h3 class="text-lg font-semibold border-b pb-2">Classification</h3>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Categorie</span></label>
                <select v-model="chaineForm.categorie" class="select select-bordered">
                  <option value="generaliste">Generaliste</option>
                  <option value="info">Info</option>
                  <option value="sport">Sport</option>
                  <option value="culture">Culture</option>
                  <option value="divertissement">Divertissement</option>
                  <option value="education">Education</option>
                  <option value="musique">Musique</option>
                </select>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Langue</span></label>
                <input v-model="chaineForm.langue" type="text" class="input input-bordered" placeholder="Ex: Francais">
              </div>
            </div>
          </div>

          <div class="space-y-4">
            <h3 class="text-lg font-semibold border-b pb-2">Localisation & diffusion</h3>
            <div class="form-control">
              <label class="label"><span class="label-text">Territoire</span></label>
              <input v-model="chaineForm.pays_id" type="text" class="input input-bordered" placeholder="UUID du territoire">
            </div>
            <div class="form-control">
              <label class="label cursor-pointer justify-start gap-3">
                <input v-model="chaineForm.est_en_direct" type="checkbox" class="checkbox checkbox-primary" />
                <span class="label-text">En direct actuellement</span>
              </label>
            </div>
          </div>

          <div class="flex justify-end gap-2 pt-4">
            <NuxtLink to="/admin/radio-tele" class="btn btn-ghost">Annuler</NuxtLink>
            <button type="submit" class="btn btn-primary" :class="{ loading }" :disabled="loading">
              <font-awesome-icon v-if="!loading" icon="plus" class="mr-1" /> Creer
            </button>
          </div>
        </form>

        <!-- Formulaire Programme Media -->
        <form v-else-if="type === 'programmes'" @submit.prevent="soumettre" class="space-y-6">
          <div class="space-y-4">
            <h3 class="text-lg font-semibold border-b pb-2">Informations de base</h3>
            <div class="form-control">
              <label class="label"><span class="label-text">Nom de l'emission *</span></label>
              <input v-model="programmeForm.nom_emission" type="text" class="input input-bordered" required placeholder="Ex: Le Grand Debat">
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Type de programme</span></label>
              <select v-model="programmeForm.type_programme" class="select select-bordered">
                <option value="radio">Radio</option>
                <option value="tele">Tele</option>
              </select>
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Description</span></label>
              <textarea v-model="programmeForm.description" class="textarea textarea-bordered h-32" placeholder="Description du programme..." />
            </div>
          </div>

          <div class="space-y-4">
            <h3 class="text-lg font-semibold border-b pb-2">Medias</h3>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">URL image de couverture</span></label>
                <input v-model="programmeForm.image_couverture_url" type="text" class="input input-bordered" placeholder="https://...">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">URL video</span></label>
                <input v-model="programmeForm.video_url" type="text" class="input input-bordered" placeholder="https://...">
              </div>
            </div>
          </div>

          <div class="space-y-4">
            <h3 class="text-lg font-semibold border-b pb-2">Equipe & production</h3>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Animateur</span></label>
                <input v-model="programmeForm.info_animateur" type="text" class="input input-bordered" placeholder="Nom de l'animateur">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Producteur</span></label>
                <input v-model="programmeForm.info_producteur" type="text" class="input input-bordered" placeholder="Nom du producteur">
              </div>
            </div>
          </div>

          <div class="space-y-4">
            <h3 class="text-lg font-semibold border-b pb-2">Classification & localisation</h3>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Territoire</span></label>
                <input v-model="programmeForm.pays_id" type="text" class="input input-bordered" placeholder="UUID du territoire">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Langue</span></label>
                <input v-model="programmeForm.langue" type="text" class="input input-bordered" placeholder="Ex: Francais">
              </div>
            </div>
            <div v-if="programmeForm.type_programme === 'radio'" class="form-control">
              <label class="label"><span class="label-text">Categorie radio</span></label>
              <select v-model="programmeForm.categorie_radio" class="select select-bordered">
                <option value="">Non specifie</option>
                <option value="information">Information</option>
                <option value="divertissement">Divertissement</option>
                <option value="musique">Musique</option>
                <option value="culture">Culture</option>
                <option value="sport">Sport</option>
                <option value="education">Education</option>
                <option value="debat">Debat</option>
                <option value="religieux">Religieux</option>
              </select>
            </div>
            <div class="form-control">
              <label class="label cursor-pointer justify-start gap-3">
                <input v-model="programmeForm.est_international" type="checkbox" class="checkbox checkbox-primary" />
                <span class="label-text">Programme international</span>
              </label>
            </div>
          </div>

          <div class="flex justify-end gap-2 pt-4">
            <NuxtLink to="/admin/radio-tele" class="btn btn-ghost">Annuler</NuxtLink>
            <button type="submit" class="btn btn-primary" :class="{ loading }" :disabled="loading">
              <font-awesome-icon v-if="!loading" icon="plus" class="mr-1" /> Creer
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>
