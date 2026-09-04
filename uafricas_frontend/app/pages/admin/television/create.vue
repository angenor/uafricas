<script setup lang="ts">
/**
 * Création d'une **chaîne TV**. Les programmes ont quitté cet écran avec la
 * migration 09q : ils vivent sur `/admin/medias/emissions`, communs aux deux
 * familles.
 *
 * Thématiques et couverture sont enregistrées **après** la création : elles
 * portent sur un support qui n'existe pas encore au moment de la saisie.
 * Une chaîne naît en brouillon, où ni l'une ni l'autre n'est exigible, le
 * refus (FR-029, FR-035) intervient à la publication, sur l'écran d'édition.
 */
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const router = useRouter()

const {
  creerChaine, ORIGINES_PUBLICATION_TELE, loading, error,
} = useAdminTelevision()
const { listerReferentielsEdition, definirThematiques, definirCouverture } = useMediaSupport()

const erreurLocale = ref<string | null>(null)

const thematiquesRef = ref<{ id: string; nom: string }[]>([])
const territoiresRef = ref<{ id: string; nom: string }[]>([])

const thematiquesChoisies = ref<string[]>([])
const couvertureContinentale = ref(false)
const territoiresChoisis = ref<string[]>([])

onMounted(async () => {
  const referentiels = await listerReferentielsEdition()
  thematiquesRef.value = referentiels.thematiques
  territoiresRef.value = referentiels.territoires
})

/**
 * Nature de la chaîne (09v). Passer en thématique vide la couverture et ramène
 * la sélection à une seule thématique : ce ne sont pas des champs masqués, ils
 * n'ont plus d'objet — le serveur refuserait la couverture d'une chaîne
 * thématique, et le trigger la seconde thématique.
 */
const estThematique = ref(false)
watch(estThematique, (valeur) => {
  if (!valeur) return
  couvertureContinentale.value = false
  territoiresChoisis.value = []
  if (thematiquesChoisies.value.length > 1) {
    thematiquesChoisies.value = thematiquesChoisies.value.slice(0, 1)
  }
})

const chaineForm = reactive({
  nom: '', description: '', stream_url: '', image_couverture_url: '',
  categorie: 'generaliste', langue: '', est_en_direct: false,
  origine_publication: 'territoire',
  contact_email: '', contact_telephone: '', contact_whatsapp: '',
  contact_site_web: '', contact_adresse: '',
})

const aideOrigine = computed(() => ORIGINES_PUBLICATION_TELE.find(o => o.valeur === chaineForm.origine_publication)?.aide || '')

const soumettre = async () => {
  erreurLocale.value = null
  try {
    if (!chaineForm.nom.trim()) { erreurLocale.value = 'Le nom de la chaîne est requis'; return }
    const body: any = {
      nom: chaineForm.nom.trim(),
      categorie: chaineForm.categorie,
      est_en_direct: chaineForm.est_en_direct,
      origine_publication: chaineForm.origine_publication,
      est_thematique: estThematique.value,
    }
    if (chaineForm.stream_url.trim()) body.stream_url = chaineForm.stream_url.trim()
    if (chaineForm.description.trim()) body.description = chaineForm.description.trim()
    if (chaineForm.image_couverture_url.trim()) body.image_couverture_url = chaineForm.image_couverture_url.trim()
    if (chaineForm.langue.trim()) body.langue = chaineForm.langue.trim()
    // Les contacts partent tels quels : le serveur les nettoie et préfixe
    // le site web, un champ vide y devenant NULL.
    body.contact_email = chaineForm.contact_email.trim()
    body.contact_telephone = chaineForm.contact_telephone.trim()
    body.contact_whatsapp = chaineForm.contact_whatsapp.trim()
    body.contact_site_web = chaineForm.contact_site_web.trim()
    body.contact_adresse = chaineForm.contact_adresse.trim()

    const cree = await creerChaine(body)

    // Thématiques et couverture ne s'écrivent qu'une fois l'identifiant connu.
    // Un échec ici ne doit pas faire croire que la chaîne n'existe pas : elle
    // est créée, l'écran d'édition permettra de compléter.
    if (cree?.id) {
      if (thematiquesChoisies.value.length) {
        await definirThematiques('chaine_tv', cree.id, thematiquesChoisies.value, true)
      }
      if (!estThematique.value && (couvertureContinentale.value || territoiresChoisis.value.length)) {
        await definirCouverture('chaine_tv', cree.id, couvertureContinentale.value, territoiresChoisis.value, true)
      }
    }
    router.push('/admin/television?type=chaines')
  }
  catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de la création'
  }
}
</script>

<template>
  <div>
    <AdminPageHeader titre="Nouvelle chaîne TV" sous-titre="Créer une chaîne de télévision">
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

        <form class="space-y-6" @submit.prevent="soumettre">
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
              <label class="label"><span class="label-text">Origine de publication *</span></label>
              <select v-model="chaineForm.origine_publication" class="select select-bordered">
                <option v-for="o in ORIGINES_PUBLICATION_TELE" :key="o.valeur" :value="o.valeur">{{ o.libelle }}</option>
              </select>
              <label class="label"><span class="label-text-alt">{{ aideOrigine }}</span></label>
            </div>
            <div class="form-control">
              <label class="label cursor-pointer justify-start gap-3">
                <input v-model="chaineForm.est_en_direct" type="checkbox" class="checkbox checkbox-primary" />
                <span class="label-text">En direct actuellement</span>
              </label>
            </div>
          </div>

          <AdminContactsSupportFields
            v-model:email="chaineForm.contact_email"
            v-model:telephone="chaineForm.contact_telephone"
            v-model:whatsapp="chaineForm.contact_whatsapp"
            v-model:site-web="chaineForm.contact_site_web"
            v-model:adresse="chaineForm.contact_adresse"
            libelle-support="la chaîne"
          />

          <div class="space-y-4">
            <h3 class="text-lg font-semibold border-b pb-2">Portée de la chaîne</h3>

            <!-- La nature commande le reste du bloc : une chaîne thématique
                 porte une thématique et couvre tous les territoires (09v). -->
            <div class="form-control">
              <label class="label cursor-pointer justify-start gap-3">
                <input v-model="estThematique" type="checkbox" class="checkbox checkbox-primary" />
                <span class="label-text">
                  Chaîne thématique — une seule thématique, sur tous les territoires
                </span>
              </label>
            </div>

            <MediaSelecteurThematiques
              v-model="thematiquesChoisies"
              :options="thematiquesRef"
              :unique="estThematique"
            />

            <MediaSelecteurCouverture
              v-if="!estThematique"
              :continentale="couvertureContinentale"
              :territoires="territoiresChoisis"
              :options="territoiresRef"
              @update:continentale="couvertureContinentale = $event"
              @update:territoires="territoiresChoisis = $event"
            />
            <p v-else class="text-sm text-base-content/60">
              Aucun territoire à saisir : une chaîne thématique les couvre tous.
            </p>

            <p class="text-sm text-base-content/60">
              Facultatives sur un brouillon ; exigées pour publier la chaîne.
            </p>
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
