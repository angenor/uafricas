<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const route = useRoute()
const router = useRouter()
const type = computed(() => (route.query.type as string) || 'chaines')

const { creerChaine, creerProgramme, listerToutesChaines, loading, error } = useAdminTelevision()
const { listerPays } = useCentresCulturels()

const erreurLocale = ref<string | null>(null)

const chainesDisponibles = ref<{ id: string; nom: string }[]>([])
const paysDisponibles = ref<{ id: string; nom: string }[]>([])
onMounted(async () => {
  const [chaines, pays] = await Promise.all([listerToutesChaines(), listerPays()])
  chainesDisponibles.value = chaines
  paysDisponibles.value = pays
  // Préremplir la chaîne si on arrive depuis la page d'une chaîne (« Ajouter une vidéo »)
  const chainePrefill = route.query.chaine as string | undefined
  if (chainePrefill && chaines.some(c => c.id === chainePrefill)) {
    programmeForm.chaine_id = chainePrefill
  }
})

const chaineForm = reactive({
  nom: '', description: '', stream_url: '', image_couverture_url: '',
  categorie: 'generaliste', pays_id: '', langue: '', est_en_direct: false,
})

const programmeForm = reactive({
  nom_emission: '', description: '', image_couverture_url: '', video_url: '',
  info_animateur: '', info_producteur: '', pays_id: '', est_international: false,
  langue: '', chaine_id: '', a_la_une: false,
})

const titreMap: Record<string, string> = { chaines: 'Nouvelle chaîne TV', programmes: 'Nouveau programme télé' }
const sousTitreMap: Record<string, string> = { chaines: 'Créer une chaîne de télévision', programmes: 'Créer un programme télé' }

const soumettre = async () => {
  erreurLocale.value = null
  try {
    if (type.value === 'chaines') {
      if (!chaineForm.nom.trim()) { erreurLocale.value = 'Le nom de la chaîne est requis'; return }
      const body: any = { nom: chaineForm.nom.trim(), categorie: chaineForm.categorie, est_en_direct: chaineForm.est_en_direct }
      if (chaineForm.stream_url.trim()) body.stream_url = chaineForm.stream_url.trim()
      if (chaineForm.description.trim()) body.description = chaineForm.description.trim()
      if (chaineForm.image_couverture_url.trim()) body.image_couverture_url = chaineForm.image_couverture_url.trim()
      if (chaineForm.pays_id) body.pays_id = chaineForm.pays_id
      if (chaineForm.langue.trim()) body.langue = chaineForm.langue.trim()
      await creerChaine(body)
      router.push('/admin/television?type=chaines')
    }
    else {
      if (!programmeForm.nom_emission.trim()) { erreurLocale.value = "Le nom du programme est requis"; return }
      const body: any = {
        nom_emission: programmeForm.nom_emission.trim(),
        est_international: programmeForm.est_international,
        a_la_une: programmeForm.chaine_id ? programmeForm.a_la_une : false,
      }
      if (programmeForm.description.trim()) body.description = programmeForm.description.trim()
      if (programmeForm.image_couverture_url.trim()) body.image_couverture_url = programmeForm.image_couverture_url.trim()
      if (programmeForm.video_url.trim()) body.video_url = programmeForm.video_url.trim()
      if (programmeForm.info_animateur.trim()) body.info_animateur = programmeForm.info_animateur.trim()
      if (programmeForm.info_producteur.trim()) body.info_producteur = programmeForm.info_producteur.trim()
      if (programmeForm.pays_id) body.pays_id = programmeForm.pays_id
      if (programmeForm.langue.trim()) body.langue = programmeForm.langue.trim()
      if (programmeForm.chaine_id) body.chaine_id = programmeForm.chaine_id
      await creerProgramme(body)
      router.push('/admin/television?type=programmes')
    }
  }
  catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de la création'
  }
}
</script>

<template>
  <div>
    <AdminPageHeader :titre="titreMap[type] || 'Nouveau'" :sous-titre="sousTitreMap[type] || ''">
      <template #actions>
        <NuxtLink to="/admin/television" class="btn btn-ghost btn-sm">
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

        <!-- Chaîne TV -->
        <form v-if="type === 'chaines'" @submit.prevent="soumettre" class="space-y-6">
          <div class="space-y-4">
            <h3 class="text-lg font-semibold border-b pb-2">Informations de base</h3>
            <div class="form-control">
              <label class="label"><span class="label-text">Nom de la chaîne *</span></label>
              <input v-model="chaineForm.nom" type="text" class="input input-bordered" required placeholder="Ex: Africa 24">
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">URL de flux live (optionnel)</span></label>
              <input v-model="chaineForm.stream_url" type="text" class="input input-bordered" placeholder="https://stream.example.com/live">
              <label class="label"><span class="label-text-alt">Le cœur de la télé = les programmes (vidéos). Le flux live est facultatif.</span></label>
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Description</span></label>
              <textarea v-model="chaineForm.description" class="textarea textarea-bordered h-32" placeholder="Description de la chaîne..." />
            </div>
            <OpportuniteAfriqueImageUploadField v-model="chaineForm.image_couverture_url" label="Image de couverture (optionnel)" />
          </div>

          <div class="space-y-4">
            <h3 class="text-lg font-semibold border-b pb-2">Classification</h3>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Catégorie</span></label>
                <select v-model="chaineForm.categorie" class="select select-bordered">
                  <option value="generaliste">Généraliste</option>
                  <option value="info">Info</option>
                  <option value="sport">Sport</option>
                  <option value="culture">Culture</option>
                  <option value="divertissement">Divertissement</option>
                  <option value="education">Éducation</option>
                  <option value="musique">Musique</option>
                </select>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Langue</span></label>
                <input v-model="chaineForm.langue" type="text" class="input input-bordered" placeholder="Ex: Français">
              </div>
            </div>
          </div>

          <div class="space-y-4">
            <h3 class="text-lg font-semibold border-b pb-2">Localisation & diffusion</h3>
            <div class="form-control">
              <label class="label"><span class="label-text">Territoire</span></label>
              <select v-model="chaineForm.pays_id" class="select select-bordered">
                <option value="">— Aucun —</option>
                <option v-for="p in paysDisponibles" :key="p.id" :value="p.id">{{ p.nom }}</option>
              </select>
            </div>
            <div class="form-control">
              <label class="label cursor-pointer justify-start gap-3">
                <input v-model="chaineForm.est_en_direct" type="checkbox" class="checkbox checkbox-primary" />
                <span class="label-text">En direct actuellement</span>
              </label>
            </div>
          </div>

          <div class="flex justify-end gap-2 pt-4">
            <NuxtLink to="/admin/television" class="btn btn-ghost">Annuler</NuxtLink>
            <button type="submit" class="btn btn-primary" :class="{ loading }" :disabled="loading">
              <font-awesome-icon v-if="!loading" icon="plus" class="mr-1" /> Créer
            </button>
          </div>
        </form>

        <!-- Programme télé -->
        <form v-else @submit.prevent="soumettre" class="space-y-6">
          <div class="space-y-4">
            <h3 class="text-lg font-semibold border-b pb-2">Informations de base</h3>
            <div class="form-control">
              <label class="label"><span class="label-text">Nom du programme *</span></label>
              <input v-model="programmeForm.nom_emission" type="text" class="input input-bordered" required placeholder="Ex: Le Grand Débat">
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Description</span></label>
              <textarea v-model="programmeForm.description" class="textarea textarea-bordered h-32" placeholder="Description du programme..." />
            </div>
          </div>

          <div class="space-y-4">
            <h3 class="text-lg font-semibold border-b pb-2">Médias</h3>
            <OpportuniteAfriqueImageUploadField v-model="programmeForm.image_couverture_url" label="Image de couverture (optionnel)" />
            <AdminMediaUploadField v-model="programmeForm.video_url" kind="video" label="Vidéo du programme (fichier ou lien)" />
          </div>

          <div class="space-y-4">
            <h3 class="text-lg font-semibold border-b pb-2">Télé & écran principal</h3>
            <div class="form-control">
              <label class="label"><span class="label-text">Télé (chaîne) de rattachement</span></label>
              <select v-model="programmeForm.chaine_id" class="select select-bordered">
                <option value="">Aucune (programme libre)</option>
                <option v-for="ch in chainesDisponibles" :key="ch.id" :value="ch.id">{{ ch.nom }}</option>
              </select>
              <label class="label"><span class="label-text-alt">Regroupe la vidéo sous cette télé sur la page publique /tele.</span></label>
            </div>
            <div class="form-control">
              <label class="label cursor-pointer justify-start gap-3">
                <input v-model="programmeForm.a_la_une" type="checkbox" class="checkbox checkbox-primary" :disabled="!programmeForm.chaine_id" />
                <span class="label-text">Programme à la une (joue en boucle sur l'écran principal de la télé)</span>
              </label>
              <label v-if="!programmeForm.chaine_id" class="label"><span class="label-text-alt text-warning">Sélectionnez d'abord une télé pour la mettre à la une.</span></label>
            </div>
          </div>

          <div class="space-y-4">
            <h3 class="text-lg font-semibold border-b pb-2">Équipe & production</h3>
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
                <select v-model="programmeForm.pays_id" class="select select-bordered">
                  <option value="">— Aucun —</option>
                  <option v-for="p in paysDisponibles" :key="p.id" :value="p.id">{{ p.nom }}</option>
                </select>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Langue</span></label>
                <input v-model="programmeForm.langue" type="text" class="input input-bordered" placeholder="Ex: Français">
              </div>
            </div>
            <div class="form-control">
              <label class="label cursor-pointer justify-start gap-3">
                <input v-model="programmeForm.est_international" type="checkbox" class="checkbox checkbox-primary" />
                <span class="label-text">Programme international</span>
              </label>
            </div>
          </div>

          <div class="flex justify-end gap-2 pt-4">
            <NuxtLink to="/admin/television" class="btn btn-ghost">Annuler</NuxtLink>
            <button type="submit" class="btn btn-primary" :class="{ loading }" :disabled="loading">
              <font-awesome-icon v-if="!loading" icon="plus" class="mr-1" /> Créer
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>
