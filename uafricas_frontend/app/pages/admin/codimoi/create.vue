<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { creer, loading, error } = useAdminCodimoi()
const router = useRouter()

const form = reactive({
  type_codimoi: 'proverbe_adage',
  contenu: '',
  explication: '',
  nom_auteur_originel: '',
  pays_id: '',
  groupe_ethnique: '',
  couleur_fond: '#FFFFFF',
  etat: 'publie',
})

const erreurLocale = ref<string | null>(null)

const soumettre = async () => {
  erreurLocale.value = null
  if (!form.contenu.trim()) {
    erreurLocale.value = 'Le contenu est requis'
    return
  }
  if (!form.type_codimoi) {
    erreurLocale.value = 'Le type est requis'
    return
  }
  try {
    const body: any = {
      type_codimoi: form.type_codimoi,
      contenu: form.contenu.trim(),
      etat: form.etat,
    }
    if (form.explication.trim()) body.explication = form.explication.trim()
    if (form.nom_auteur_originel.trim()) body.nom_auteur_originel = form.nom_auteur_originel.trim()
    if (form.pays_id.trim()) body.pays_id = form.pays_id.trim()
    if (form.groupe_ethnique.trim()) body.groupe_ethnique = form.groupe_ethnique.trim()
    if (form.couleur_fond && form.couleur_fond !== '#FFFFFF') body.couleur_fond = form.couleur_fond
    await creer(body)
    router.push('/admin/codimoi')
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de la creation'
  }
}
</script>

<template>
  <div>
    <AdminPageHeader titre="Nouveau codi-moi" sous-titre="Ajouter un proverbe, citation ou ressource culturelle">
      <template #actions>
        <NuxtLink to="/admin/codimoi" class="btn btn-ghost btn-sm">
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

        <form @submit.prevent="soumettre" class="space-y-4">
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Type *</span></label>
              <select v-model="form.type_codimoi" class="select select-bordered" required>
                <option value="proverbe_adage">Proverbe / Adage</option>
                <option value="citation">Citation</option>
                <option value="ressource_historique">Ressource historique</option>
                <option value="bonne_pratique">Bonne pratique</option>
              </select>
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">État</span></label>
              <select v-model="form.etat" class="select select-bordered">
                <option value="publie">Publie</option>
                <option value="brouillon">Brouillon</option>
              </select>
            </div>
          </div>

          <div class="form-control">
            <label class="label"><span class="label-text">Contenu *</span></label>
            <textarea v-model="form.contenu" class="textarea textarea-bordered" rows="4" required placeholder="Texte du proverbe, citation ou ressource..." />
          </div>

          <div class="form-control">
            <label class="label"><span class="label-text">Explication</span></label>
            <textarea v-model="form.explication" class="textarea textarea-bordered" rows="3" placeholder="Explication ou contexte culturel..." />
          </div>

          <div v-if="form.type_codimoi === 'citation'" class="form-control">
            <label class="label"><span class="label-text">Auteur originel</span></label>
            <input v-model="form.nom_auteur_originel" type="text" class="input input-bordered" placeholder="Nom de l'auteur de la citation">
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Territoire (UUID)</span></label>
              <input v-model="form.pays_id" type="text" class="input input-bordered" placeholder="ID du territoire d'origine">
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Groupe ethnique</span></label>
              <input v-model="form.groupe_ethnique" type="text" class="input input-bordered" placeholder="Ex: Bamileke, Wolof, Zulu...">
            </div>
          </div>

          <div class="form-control">
            <label class="label"><span class="label-text">Couleur de fond</span></label>
            <div class="flex items-center gap-3">
              <input v-model="form.couleur_fond" type="color" class="w-12 h-10 rounded cursor-pointer border border-base-300">
              <input v-model="form.couleur_fond" type="text" class="input input-bordered input-sm w-32" placeholder="#FFFFFF">
            </div>
          </div>

          <div class="flex justify-end gap-2 pt-4">
            <NuxtLink to="/admin/codimoi" class="btn btn-ghost">Annuler</NuxtLink>
            <button type="submit" class="btn btn-primary" :class="{ loading }" :disabled="loading">
              <font-awesome-icon v-if="!loading" icon="plus" class="mr-1" /> Creer
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>
