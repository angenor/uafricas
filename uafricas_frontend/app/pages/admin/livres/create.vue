<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { creer, loading, error } = useAdminLivres()
const router = useRouter()

const erreurLocale = ref<string | null>(null)

const form = reactive({
  titre: '',
  info_auteur: '',
  description: '',
  document_pdf_url: '',
  type_document: 'livre',
  image_couverture_url: '',
  acces: 'gratuit',
  categorie_id: '',
  langue: '',
  date_publication: '',
  isbn: '',
  nombre_pages: null as number | null,
  rapport_auteur: '',
  condition_diffusion: '',
  acceptation_diffusion: false,
})

const soumettre = async () => {
  erreurLocale.value = null
  if (!form.titre.trim()) {
    erreurLocale.value = 'Le titre est requis'
    return
  }
  if (!form.document_pdf_url.trim()) {
    erreurLocale.value = 'L\'URL du document PDF est requise'
    return
  }
  try {
    const body: any = {
      titre: form.titre.trim(),
      document_pdf_url: form.document_pdf_url.trim(),
      type_document: form.type_document,
      acces: form.acces,
      acceptation_diffusion: form.acceptation_diffusion,
    }
    if (form.info_auteur.trim()) body.info_auteur = form.info_auteur.trim()
    if (form.description.trim()) body.description = form.description.trim()
    if (form.image_couverture_url.trim()) body.image_couverture_url = form.image_couverture_url.trim()
    if (form.categorie_id.trim()) body.categorie_id = form.categorie_id.trim()
    if (form.langue.trim()) body.langue = form.langue.trim()
    if (form.date_publication) body.date_publication = form.date_publication
    if (form.isbn.trim()) body.isbn = form.isbn.trim()
    if (form.nombre_pages !== null && form.nombre_pages > 0) body.nombre_pages = form.nombre_pages
    if (form.rapport_auteur.trim()) body.rapport_auteur = form.rapport_auteur.trim()
    if (form.condition_diffusion.trim()) body.condition_diffusion = form.condition_diffusion.trim()

    await creer(body)
    router.push('/admin/livres')
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de la creation'
  }
}
</script>

<template>
  <div>
    <AdminPageHeader titre="Nouveau livre" sous-titre="Ajouter un document a la bibliotheque">
      <template #actions>
        <NuxtLink to="/admin/livres" class="btn btn-ghost btn-sm">
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
          <!-- Section 1 : Infos de base -->
          <div class="divider">Informations de base</div>

          <div class="form-control">
            <label class="label"><span class="label-text">Titre *</span></label>
            <input v-model="form.titre" type="text" class="input input-bordered" placeholder="Titre du document" required />
          </div>

          <div class="form-control">
            <label class="label"><span class="label-text">Auteur</span></label>
            <input v-model="form.info_auteur" type="text" class="input input-bordered" placeholder="Nom de l'auteur" />
          </div>

          <div class="form-control">
            <label class="label"><span class="label-text">Description</span></label>
            <textarea v-model="form.description" class="textarea textarea-bordered" rows="4" placeholder="Description du document..." />
          </div>

          <!-- Section 2 : Document -->
          <div class="divider">Document</div>

          <div class="form-control">
            <label class="label"><span class="label-text">URL du document PDF *</span></label>
            <input v-model="form.document_pdf_url" type="text" class="input input-bordered" placeholder="https://..." required />
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Type de document</span></label>
              <select v-model="form.type_document" class="select select-bordered">
                <option value="livre">Livre</option>
                <option value="article">Article</option>
                <option value="rapport">Rapport</option>
                <option value="these">These</option>
                <option value="memoire">Memoire</option>
                <option value="revue">Revue</option>
              </select>
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Image de couverture (URL)</span></label>
              <input v-model="form.image_couverture_url" type="text" class="input input-bordered" placeholder="https://..." />
            </div>
          </div>

          <!-- Section 3 : Acces & Classification -->
          <div class="divider">Acces & Classification</div>

          <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Acces</span></label>
              <select v-model="form.acces" class="select select-bordered">
                <option value="gratuit">Gratuit</option>
                <option value="premium">Premium</option>
                <option value="restreint">Restreint</option>
              </select>
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Catégorie (UUID)</span></label>
              <input v-model="form.categorie_id" type="text" class="input input-bordered" placeholder="UUID de la catégorie" />
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Langue</span></label>
              <input v-model="form.langue" type="text" class="input input-bordered" placeholder="Ex: fr, en, ar..." />
            </div>
          </div>

          <!-- Section 4 : Publication -->
          <div class="divider">Publication</div>

          <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Date de publication</span></label>
              <input v-model="form.date_publication" type="date" class="input input-bordered" />
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">ISBN</span></label>
              <input v-model="form.isbn" type="text" class="input input-bordered" placeholder="978-..." />
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Nombre de pages</span></label>
              <input v-model.number="form.nombre_pages" type="number" class="input input-bordered" min="1" placeholder="0" />
            </div>
          </div>

          <!-- Section 5 : Diffusion -->
          <div class="divider">Diffusion</div>

          <div class="form-control">
            <label class="label"><span class="label-text">Rapport de l'auteur</span></label>
            <textarea v-model="form.rapport_auteur" class="textarea textarea-bordered" rows="3" placeholder="Rapport ou note de l'auteur..." />
          </div>

          <div class="form-control">
            <label class="label"><span class="label-text">Conditions de diffusion</span></label>
            <textarea v-model="form.condition_diffusion" class="textarea textarea-bordered" rows="3" placeholder="Conditions de diffusion du document..." />
          </div>

          <div class="form-control">
            <label class="label cursor-pointer justify-start gap-3">
              <input v-model="form.acceptation_diffusion" type="checkbox" class="checkbox checkbox-primary" />
              <span class="label-text">L'auteur accepte la diffusion sur la plateforme</span>
            </label>
          </div>

          <!-- Actions -->
          <div class="flex justify-end gap-2 pt-4 border-t">
            <NuxtLink to="/admin/livres" class="btn btn-ghost">Annuler</NuxtLink>
            <button type="submit" class="btn btn-primary" :class="{ loading }" :disabled="loading">
              <font-awesome-icon v-if="!loading" icon="plus" class="mr-1" /> Creer le document
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>
