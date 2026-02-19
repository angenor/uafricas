<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { creer, loading, error } = useAdminEvenements()
const router = useRouter()

const form = reactive({
  titre: '',
  description: '',
  type_evenement: '',
  format: 'presentiel',
  langue: 'fr',
  date_heure_debut: '',
  date_heure_fin: '',
  nombre_places: null as number | null,
  pays_id: '',
  ville: '',
  adresse: '',
  lien_en_ligne: '',
  image_couverture_url: '',
})

const erreurLocale = ref<string | null>(null)

const afficherLienEnLigne = computed(() => form.format === 'en_ligne' || form.format === 'hybride')

const soumettre = async () => {
  erreurLocale.value = null
  if (!form.titre.trim()) {
    erreurLocale.value = 'Le titre de l\'evenement est requis'
    return
  }
  if (!form.date_heure_debut) {
    erreurLocale.value = 'La date de debut est requise'
    return
  }
  try {
    const body: any = {
      titre: form.titre.trim(),
      format: form.format,
      langue: form.langue,
      date_heure_debut: form.date_heure_debut,
    }
    if (form.description.trim()) body.description = form.description.trim()
    if (form.type_evenement.trim()) body.type_evenement = form.type_evenement.trim()
    if (form.date_heure_fin) body.date_heure_fin = form.date_heure_fin
    if (form.nombre_places !== null) body.nombre_places = form.nombre_places
    if (form.pays_id.trim()) body.pays_id = form.pays_id.trim()
    if (form.ville.trim()) body.ville = form.ville.trim()
    if (form.adresse.trim()) body.adresse = form.adresse.trim()
    if (form.lien_en_ligne.trim()) body.lien_en_ligne = form.lien_en_ligne.trim()
    if (form.image_couverture_url.trim()) body.image_couverture_url = form.image_couverture_url.trim()

    await creer(body)
    router.push('/admin/evenements')
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de la creation'
  }
}
</script>

<template>
  <div>
    <AdminPageHeader titre="Nouvel evenement" sous-titre="Creer un evenement">
      <template #actions>
        <NuxtLink to="/admin/evenements" class="btn btn-ghost btn-sm">
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

        <form @submit.prevent="soumettre" class="space-y-6">
          <!-- Infos de base -->
          <div class="space-y-4">
            <h3 class="text-lg font-semibold border-b pb-2">Informations de base</h3>
            <div class="form-control">
              <label class="label"><span class="label-text">Titre de l'evenement *</span></label>
              <input v-model="form.titre" type="text" class="input input-bordered" required placeholder="Ex: Conference sur le developpement durable">
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Description</span></label>
              <textarea v-model="form.description" class="textarea textarea-bordered h-32" placeholder="Description detaillee de l'evenement, objectifs, programme..." />
            </div>
          </div>

          <!-- Type & format -->
          <div class="space-y-4">
            <h3 class="text-lg font-semibold border-b pb-2">Type & format</h3>
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Type d'evenement</span></label>
                <input v-model="form.type_evenement" type="text" class="input input-bordered" placeholder="Ex: Conference, Atelier, Seminaire">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Format</span></label>
                <select v-model="form.format" class="select select-bordered">
                  <option value="presentiel">Presentiel</option>
                  <option value="en_ligne">En ligne</option>
                  <option value="hybride">Hybride</option>
                </select>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Langue</span></label>
                <input v-model="form.langue" type="text" class="input input-bordered" placeholder="Ex: fr, en, ar">
              </div>
            </div>
          </div>

          <!-- Dates -->
          <div class="space-y-4">
            <h3 class="text-lg font-semibold border-b pb-2">Dates & capacite</h3>
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Date et heure de debut *</span></label>
                <input v-model="form.date_heure_debut" type="datetime-local" class="input input-bordered" required>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Date et heure de fin</span></label>
                <input v-model="form.date_heure_fin" type="datetime-local" class="input input-bordered">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Nombre de places</span></label>
                <input v-model.number="form.nombre_places" type="number" min="1" class="input input-bordered" placeholder="Illimite si vide">
              </div>
            </div>
          </div>

          <!-- Localisation -->
          <div class="space-y-4">
            <h3 class="text-lg font-semibold border-b pb-2">Localisation</h3>
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Pays</span></label>
                <input v-model="form.pays_id" type="text" class="input input-bordered" placeholder="UUID du pays">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Ville</span></label>
                <input v-model="form.ville" type="text" class="input input-bordered" placeholder="Ex: Dakar">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Adresse</span></label>
                <input v-model="form.adresse" type="text" class="input input-bordered" placeholder="Adresse complete">
              </div>
            </div>
            <div v-if="afficherLienEnLigne" class="form-control">
              <label class="label"><span class="label-text">Lien en ligne</span></label>
              <input v-model="form.lien_en_ligne" type="url" class="input input-bordered" placeholder="https://zoom.us/j/...">
            </div>
          </div>

          <!-- Medias -->
          <div class="space-y-4">
            <h3 class="text-lg font-semibold border-b pb-2">Image de couverture</h3>
            <div class="form-control">
              <label class="label"><span class="label-text">URL de l'image de couverture</span></label>
              <input v-model="form.image_couverture_url" type="url" class="input input-bordered" placeholder="https://exemple.com/image.jpg">
            </div>
          </div>

          <div class="flex justify-end gap-2 pt-4">
            <NuxtLink to="/admin/evenements" class="btn btn-ghost">Annuler</NuxtLink>
            <button type="submit" class="btn btn-primary" :class="{ loading }" :disabled="loading">
              <font-awesome-icon v-if="!loading" icon="plus" class="mr-1" /> Creer
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>
