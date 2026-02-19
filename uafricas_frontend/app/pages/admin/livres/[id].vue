<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const route = useRoute()
const id = route.params.id as string
const {
  livreDetail, chargerDetail, modifier, changerEtat,
  ajouterTag, retirerTag,
  loading, error,
} = useAdminLivres()

const saving = ref(false)
const erreurLocale = ref<string | null>(null)
const successMsg = ref<string | null>(null)
const ongletActif = ref('infos')

// Formulaire principal
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

// Etat moderation
const etatLivre = ref('')
const etatLoading = ref(false)

// Tags
const nouveauTagId = ref('')
const tagLoading = ref(false)

const charger = async () => {
  await chargerDetail(id)
  if (livreDetail.value) {
    const l = livreDetail.value
    form.titre = l.titre
    form.info_auteur = l.info_auteur || ''
    form.description = l.description || ''
    form.document_pdf_url = l.document_pdf_url || ''
    form.type_document = l.type_document || 'livre'
    form.image_couverture_url = l.image_couverture_url || ''
    form.acces = l.acces || 'gratuit'
    form.categorie_id = l.categorie_id || ''
    form.langue = l.langue || ''
    form.date_publication = l.date_publication || ''
    form.isbn = l.isbn || ''
    form.nombre_pages = l.nombre_pages
    form.rapport_auteur = l.rapport_auteur || ''
    form.condition_diffusion = l.condition_diffusion || ''
    form.acceptation_diffusion = l.acceptation_diffusion || false
    etatLivre.value = l.etat
  }
}

const sauvegarder = async () => {
  saving.value = true
  erreurLocale.value = null
  successMsg.value = null
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

    await modifier(id, body)
    successMsg.value = 'Document mis a jour avec succes'
    setTimeout(() => { successMsg.value = null }, 3000)
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de la sauvegarde'
  } finally {
    saving.value = false
  }
}

const executerChangerEtat = async (nouvelEtat: string) => {
  etatLoading.value = true
  try {
    await changerEtat(id, nouvelEtat)
    etatLivre.value = nouvelEtat
    successMsg.value = `Etat change en "${nouvelEtat}"`
    setTimeout(() => { successMsg.value = null }, 3000)
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  } finally {
    etatLoading.value = false
  }
}

// Tags
const ajouterTagHandler = async () => {
  if (!nouveauTagId.value.trim()) return
  tagLoading.value = true
  try {
    await ajouterTag(id, nouveauTagId.value.trim())
    nouveauTagId.value = ''
    await chargerDetail(id)
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  } finally {
    tagLoading.value = false
  }
}

const supprimerTagHandler = async (tagId: string) => {
  try {
    await retirerTag(id, tagId)
    await chargerDetail(id)
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  }
}

const etatBadge = (etat: string) => {
  const map: Record<string, string> = {
    publie: 'badge-success',
    brouillon: 'badge-info',
    suspendu: 'badge-error',
  }
  return map[etat] || 'badge-neutral'
}

const accesBadge = (acces: string) => {
  const map: Record<string, string> = {
    gratuit: 'badge-success',
    premium: 'badge-warning',
    restreint: 'badge-error',
  }
  return map[acces] || 'badge-neutral'
}

onMounted(() => charger())
</script>

<template>
  <div>
    <AdminPageHeader :titre="livreDetail?.titre || 'Chargement...'" sous-titre="Modifier le document">
      <template #actions>
        <NuxtLink to="/admin/livres" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Retour
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div v-if="loading && !livreDetail" class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <div v-else-if="livreDetail" class="space-y-4">
      <!-- Alertes -->
      <div v-if="erreurLocale || error" class="alert alert-error">
        <font-awesome-icon icon="circle-exclamation" />
        <span>{{ erreurLocale || error }}</span>
        <button class="btn btn-ghost btn-xs" @click="erreurLocale = null">
          <font-awesome-icon icon="xmark" />
        </button>
      </div>
      <div v-if="successMsg" class="alert alert-success">
        <font-awesome-icon icon="circle-check" />
        <span>{{ successMsg }}</span>
      </div>

      <!-- Barre d'etat + moderation -->
      <div class="card bg-base-100 shadow-sm">
        <div class="card-body flex-row items-center justify-between py-3">
          <div class="flex items-center gap-3">
            <span class="text-sm font-medium">Etat :</span>
            <span class="badge" :class="etatBadge(etatLivre)">{{ etatLivre }}</span>
            <span class="text-sm font-medium ml-4">Acces :</span>
            <span class="badge" :class="accesBadge(livreDetail.acces)">{{ livreDetail.acces }}</span>
          </div>
          <div class="flex gap-2">
            <button
              v-if="etatLivre !== 'publie'"
              class="btn btn-success btn-sm"
              :class="{ loading: etatLoading }"
              :disabled="etatLoading"
              @click="executerChangerEtat('publie')"
            >
              Publier
            </button>
            <button
              v-if="etatLivre === 'publie'"
              class="btn btn-warning btn-sm"
              :class="{ loading: etatLoading }"
              :disabled="etatLoading"
              @click="executerChangerEtat('suspendu')"
            >
              Suspendre
            </button>
            <button
              v-if="etatLivre !== 'brouillon'"
              class="btn btn-ghost btn-sm"
              :class="{ loading: etatLoading }"
              :disabled="etatLoading"
              @click="executerChangerEtat('brouillon')"
            >
              Brouillon
            </button>
          </div>
        </div>
      </div>

      <!-- Onglets -->
      <div role="tablist" class="tabs tabs-bordered">
        <button role="tab" class="tab" :class="{ 'tab-active': ongletActif === 'infos' }" @click="ongletActif = 'infos'">
          <font-awesome-icon icon="circle-info" class="mr-1" /> Informations
        </button>
        <button role="tab" class="tab" :class="{ 'tab-active': ongletActif === 'tags' }" @click="ongletActif = 'tags'">
          <font-awesome-icon icon="tags" class="mr-1" /> Tags ({{ livreDetail.tags.length }})
        </button>
      </div>

      <!-- Onglet Informations -->
      <div v-show="ongletActif === 'infos'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <form @submit.prevent="sauvegarder" class="space-y-4">
            <!-- Stats lecture seule -->
            <div class="stats stats-horizontal shadow w-full mb-4">
              <div class="stat">
                <div class="stat-title">Vues</div>
                <div class="stat-value text-lg">{{ livreDetail.nombre_vues }}</div>
              </div>
              <div class="stat">
                <div class="stat-title">Telechargements</div>
                <div class="stat-value text-lg">{{ livreDetail.nombre_telechargements }}</div>
              </div>
            </div>

            <!-- Infos de base -->
            <div class="divider">Informations de base</div>

            <div class="form-control">
              <label class="label"><span class="label-text">Titre *</span></label>
              <input v-model="form.titre" type="text" class="input input-bordered" required />
            </div>

            <div class="form-control">
              <label class="label"><span class="label-text">Auteur</span></label>
              <input v-model="form.info_auteur" type="text" class="input input-bordered" />
            </div>

            <div class="form-control">
              <label class="label"><span class="label-text">Description</span></label>
              <textarea v-model="form.description" class="textarea textarea-bordered" rows="4" />
            </div>

            <!-- Document -->
            <div class="divider">Document</div>

            <div class="form-control">
              <label class="label"><span class="label-text">URL du document PDF *</span></label>
              <input v-model="form.document_pdf_url" type="text" class="input input-bordered" required />
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

            <!-- Acces & Classification -->
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
                <label class="label"><span class="label-text">Categorie (UUID)</span></label>
                <input v-model="form.categorie_id" type="text" class="input input-bordered" placeholder="UUID de la categorie" />
                <label v-if="livreDetail.categorie_nom" class="label">
                  <span class="label-text-alt">Actuelle : {{ livreDetail.categorie_nom }}</span>
                </label>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Langue</span></label>
                <input v-model="form.langue" type="text" class="input input-bordered" placeholder="Ex: fr, en, ar..." />
              </div>
            </div>

            <!-- Publication -->
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
                <input v-model.number="form.nombre_pages" type="number" class="input input-bordered" min="1" />
              </div>
            </div>

            <!-- Diffusion -->
            <div class="divider">Diffusion</div>

            <div class="form-control">
              <label class="label"><span class="label-text">Rapport de l'auteur</span></label>
              <textarea v-model="form.rapport_auteur" class="textarea textarea-bordered" rows="3" />
            </div>

            <div class="form-control">
              <label class="label"><span class="label-text">Conditions de diffusion</span></label>
              <textarea v-model="form.condition_diffusion" class="textarea textarea-bordered" rows="3" />
            </div>

            <div class="form-control">
              <label class="label cursor-pointer justify-start gap-3">
                <input v-model="form.acceptation_diffusion" type="checkbox" class="checkbox checkbox-primary" />
                <span class="label-text">L'auteur accepte la diffusion sur la plateforme</span>
              </label>
            </div>

            <!-- Metadonnees + Sauvegarder -->
            <div class="flex items-center justify-between pt-4 border-t">
              <div class="text-sm text-base-content/50">
                <span v-if="livreDetail.slug">Slug : {{ livreDetail.slug }}</span>
                <span class="ml-4">Par : {{ livreDetail.cree_par_nom }}</span>
                <span class="ml-4">Cree le : {{ new Date(livreDetail.created_at).toLocaleDateString('fr-FR') }}</span>
                <span v-if="livreDetail.updated_at" class="ml-4">Modifie le : {{ new Date(livreDetail.updated_at).toLocaleDateString('fr-FR') }}</span>
              </div>
              <button type="submit" class="btn btn-primary" :class="{ loading: saving }" :disabled="saving">
                <font-awesome-icon v-if="!saving" icon="floppy-disk" class="mr-1" /> Enregistrer
              </button>
            </div>
          </form>
        </div>
      </div>

      <!-- Onglet Tags -->
      <div v-show="ongletActif === 'tags'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <h3 class="font-semibold mb-4">Tags associes</h3>
          <table class="table table-sm">
            <thead>
              <tr>
                <th>Tag</th>
                <th class="w-16">Actions</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="tag in livreDetail.tags" :key="tag.tag_id">
                <td><span class="badge badge-sm badge-outline">{{ tag.tag_nom }}</span></td>
                <td>
                  <button class="btn btn-ghost btn-xs text-error" @click="supprimerTagHandler(tag.tag_id)">
                    <font-awesome-icon icon="trash" />
                  </button>
                </td>
              </tr>
              <tr v-if="!livreDetail.tags.length">
                <td colspan="2" class="text-center text-base-content/50 py-4">Aucun tag associe</td>
              </tr>
            </tbody>
          </table>
          <div class="flex gap-2 mt-4">
            <input
              v-model="nouveauTagId"
              type="text"
              class="input input-bordered input-sm flex-1"
              placeholder="UUID du tag a ajouter"
            />
            <button
              class="btn btn-primary btn-sm"
              :class="{ loading: tagLoading }"
              :disabled="tagLoading || !nouveauTagId.trim()"
              @click="ajouterTagHandler"
            >
              <font-awesome-icon icon="plus" class="mr-1" /> Ajouter
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
