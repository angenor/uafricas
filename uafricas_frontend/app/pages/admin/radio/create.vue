<script setup lang="ts">
/**
 * Création d'une **station de radio**. Les émissions ont quitté cet écran avec
 * la migration 09q : elles vivent sur `/admin/medias/emissions`, communes aux
 * deux familles.
 *
 * Thématiques et couverture s'écrivent **après** la création — elles portent
 * sur un support qui n'existe pas encore au moment de la saisie.
 */
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const router = useRouter()

const {
  creerStation, loading, error,
  ORIGINES_PUBLICATION_RADIO, ROLES_PARTIE_PRENANTE_RADIO,
} = useAdminRadio()
const { listerPays } = useCentresCulturels()
const { listerReferentielsEdition, definirThematiques, definirCouverture } = useMediaSupport()

const erreurLocale = ref<string | null>(null)

const paysDisponibles = ref<{ id: string; nom: string }[]>([])
const thematiquesRef = ref<{ id: string; nom: string }[]>([])
const territoiresRef = ref<{ id: string; nom: string }[]>([])

const thematiquesChoisies = ref<string[]>([])
const couvertureContinentale = ref(false)
const territoiresChoisis = ref<string[]>([])

onMounted(async () => {
  const [pays, referentiels] = await Promise.all([listerPays(), listerReferentielsEdition()])
  paysDisponibles.value = pays
  thematiquesRef.value = referentiels.thematiques
  territoiresRef.value = referentiels.territoires
})

// Station form
const stationForm = reactive({
  nom: '', description: '', stream_url: '', audio_url: '', image_couverture_url: '',
  genre: '', genres_liste: [] as string[], pays_id: '', ville: '', type_station: 'nationale', a_la_une: false,
  origine_publication: 'territoire', role_partie_prenante: '', role_partie_prenante_autre: '',
  contact_email: '', contact_telephone: '', contact_whatsapp: '',
  contact_site_web: '', contact_adresse: '',
})

const aideOrigine = computed(() => ORIGINES_PUBLICATION_RADIO.find(o => o.valeur === stationForm.origine_publication)?.aide || '')

const genreInput = ref('')
const ajouterGenre = () => {
  const g = genreInput.value.trim()
  if (g && !stationForm.genres_liste.includes(g)) { stationForm.genres_liste.push(g); genreInput.value = '' }
}
const retirerGenre = (i: number) => stationForm.genres_liste.splice(i, 1)

const soumettre = async () => {
  erreurLocale.value = null
  try {
    if (!stationForm.nom.trim()) { erreurLocale.value = 'Le nom de la station est requis'; return }
    if (!stationForm.audio_url.trim() && !stationForm.stream_url.trim()) {
      erreurLocale.value = 'Fournissez un fichier/lien audio ou une URL de flux live'; return
    }
    if (stationForm.role_partie_prenante === 'autre' && !stationForm.role_partie_prenante_autre.trim()) {
      erreurLocale.value = 'Précisez le rôle lorsque « Autre » est sélectionné'; return
    }
    const body: any = {
      nom: stationForm.nom.trim(),
      type_station: stationForm.type_station,
      a_la_une: stationForm.a_la_une,
      origine_publication: stationForm.origine_publication,
    }
    if (stationForm.role_partie_prenante) {
      body.role_partie_prenante = stationForm.role_partie_prenante
      if (stationForm.role_partie_prenante === 'autre') body.role_partie_prenante_autre = stationForm.role_partie_prenante_autre.trim()
    }
    if (stationForm.stream_url.trim()) body.stream_url = stationForm.stream_url.trim()
    if (stationForm.audio_url.trim()) body.audio_url = stationForm.audio_url.trim()
    if (stationForm.description.trim()) body.description = stationForm.description.trim()
    if (stationForm.image_couverture_url.trim()) body.image_couverture_url = stationForm.image_couverture_url.trim()
    if (stationForm.genre.trim()) body.genre = stationForm.genre.trim()
    if (stationForm.genres_liste.length) body.genres_liste = stationForm.genres_liste
    if (stationForm.pays_id) body.pays_id = stationForm.pays_id
    if (stationForm.ville.trim()) body.ville = stationForm.ville.trim()
    // Les contacts partent tels quels : le serveur les nettoie et préfixe
    // le site web, un champ vide y devenant NULL.
    body.contact_email = stationForm.contact_email.trim()
    body.contact_telephone = stationForm.contact_telephone.trim()
    body.contact_whatsapp = stationForm.contact_whatsapp.trim()
    body.contact_site_web = stationForm.contact_site_web.trim()
    body.contact_adresse = stationForm.contact_adresse.trim()

    const cree = await creerStation(body)

    // Écrites seulement une fois l'identifiant connu ; un échec ici ne signifie
    // pas que la station n'existe pas — l'écran d'édition permet de compléter.
    if (cree?.id) {
      if (thematiquesChoisies.value.length) {
        await definirThematiques('station_radio', cree.id, thematiquesChoisies.value, true)
      }
      if (couvertureContinentale.value || territoiresChoisis.value.length) {
        await definirCouverture('station_radio', cree.id, couvertureContinentale.value, territoiresChoisis.value, true)
      }
    }
    router.push('/admin/radio?type=stations')
  }
  catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de la création'
  }
}
</script>

<template>
  <div>
    <AdminPageHeader titre="Nouvelle station radio" sous-titre="Créer une station de radio">
      <template #actions>
        <NuxtLink to="/admin/radio" class="btn btn-ghost btn-sm">
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

        <form class="space-y-6" @submit.prevent="soumettre">
          <div class="space-y-4">
            <h3 class="text-lg font-semibold border-b pb-2">Informations de base</h3>
            <div class="form-control">
              <label class="label"><span class="label-text">Nom de la station *</span></label>
              <input v-model="stationForm.nom" type="text" class="input input-bordered" required placeholder="Ex: Radio Africa 1">
            </div>

            <AdminMediaUploadField v-model="stationForm.audio_url" kind="audio" label="Audio de la station (fichier ou lien)" />

            <div class="form-control">
              <label class="label"><span class="label-text">URL de flux live (optionnel)</span></label>
              <input v-model="stationForm.stream_url" type="text" class="input input-bordered" placeholder="https://stream.example.com/live">
              <label class="label"><span class="label-text-alt">Diffusion en direct facultative, en complément de l'audio.</span></label>
            </div>

            <div class="form-control">
              <label class="label"><span class="label-text">Description</span></label>
              <textarea v-model="stationForm.description" class="textarea textarea-bordered h-32" placeholder="Description de la station..." />
            </div>

            <OpportuniteAfriqueImageUploadField v-model="stationForm.image_couverture_url" label="Image de couverture (optionnel)" />

            <div class="form-control">
              <label class="label cursor-pointer justify-start gap-3">
                <input v-model="stationForm.a_la_une" type="checkbox" class="checkbox checkbox-primary" />
                <span class="label-text">À la une (station mise en avant sur sa page)</span>
              </label>
            </div>
          </div>

          <div class="space-y-4">
            <h3 class="text-lg font-semibold border-b pb-2">Publication &amp; rôle</h3>
            <div class="form-control">
              <label class="label"><span class="label-text">Origine de publication *</span></label>
              <select v-model="stationForm.origine_publication" class="select select-bordered">
                <option v-for="o in ORIGINES_PUBLICATION_RADIO" :key="o.valeur" :value="o.valeur">
                  {{ o.libelle }} — page {{ o.page }}
                </option>
              </select>
              <label class="label"><span class="label-text-alt">{{ aideOrigine }}</span></label>
            </div>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Rôle de la partie prenante</span></label>
                <select v-model="stationForm.role_partie_prenante" class="select select-bordered">
                  <option value="">— Non spécifié —</option>
                  <option v-for="r in ROLES_PARTIE_PRENANTE_RADIO" :key="r.valeur" :value="r.valeur">{{ r.libelle }}</option>
                </select>
              </div>
              <div v-if="stationForm.role_partie_prenante === 'autre'" class="form-control">
                <label class="label"><span class="label-text">Préciser le rôle *</span></label>
                <input v-model="stationForm.role_partie_prenante_autre" type="text" class="input input-bordered" required placeholder="Ex: Collectif de podcasteurs">
              </div>
            </div>
          </div>

          <AdminContactsSupportFields
            v-model:email="stationForm.contact_email"
            v-model:telephone="stationForm.contact_telephone"
            v-model:whatsapp="stationForm.contact_whatsapp"
            v-model:site-web="stationForm.contact_site_web"
            v-model:adresse="stationForm.contact_adresse"
            libelle-support="la station"
          />

          <div class="space-y-4">
            <h3 class="text-lg font-semibold border-b pb-2">Classification</h3>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Genre principal</span></label>
                <input v-model="stationForm.genre" type="text" class="input input-bordered" placeholder="Ex: Musique, Actualités">
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
                <select v-model="stationForm.pays_id" class="select select-bordered">
                  <option value="">— Aucun —</option>
                  <option v-for="p in paysDisponibles" :key="p.id" :value="p.id">{{ p.nom }}</option>
                </select>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Ville</span></label>
                <input v-model="stationForm.ville" type="text" class="input input-bordered" placeholder="Ex: Dakar">
              </div>
            </div>
          </div>

          <div class="space-y-4">
            <h3 class="text-lg font-semibold border-b pb-2">Thématiques &amp; couverture</h3>
            <MediaSelecteurThematiques
              v-model="thematiquesChoisies"
              :options="thematiquesRef"
            />
            <MediaSelecteurCouverture
              :continentale="couvertureContinentale"
              :territoires="territoiresChoisis"
              :options="territoiresRef"
              @update:continentale="couvertureContinentale = $event"
              @update:territoires="territoiresChoisis = $event"
            />
            <p class="text-sm text-base-content/60">
              Facultatives sur un brouillon ; exigées pour publier la station.
            </p>
          </div>

          <div class="flex justify-end gap-2 pt-4">
            <NuxtLink to="/admin/radio" class="btn btn-ghost">Annuler</NuxtLink>
            <button type="submit" class="btn btn-primary" :class="{ loading }" :disabled="loading">
              <font-awesome-icon v-if="!loading" icon="plus" class="mr-1" /> Créer
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>
