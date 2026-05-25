<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { creer, loading, error } = useAdminFactcheck()
const router = useRouter()

const form = reactive({
  contenu: '',
  source_originale: '',
  verdict: 'non_verifie',
  pays_id: '',
  couleur_fond: '#FFFFFF',
  etat: 'brouillon',
})

const erreurLocale = ref<string | null>(null)

const soumettre = async () => {
  erreurLocale.value = null
  if (!form.contenu.trim()) {
    erreurLocale.value = 'Le contenu est requis'
    return
  }
  try {
    const body: any = {
      contenu: form.contenu.trim(),
      verdict: form.verdict,
      etat: form.etat,
    }
    if (form.source_originale.trim()) body.source_originale = form.source_originale.trim()
    if (form.pays_id.trim()) body.pays_id = form.pays_id.trim()
    if (form.couleur_fond && form.couleur_fond !== '#FFFFFF') body.couleur_fond = form.couleur_fond
    await creer(body)
    router.push('/admin/factcheck')
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de la creation'
  }
}
</script>

<template>
  <div>
    <AdminPageHeader titre="Nouveau factcheck" sous-titre="Ajouter une verification d'affirmation">
      <template #actions>
        <NuxtLink to="/admin/factcheck" class="btn btn-ghost btn-sm">
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
          <div class="form-control">
            <label class="label"><span class="label-text">Contenu *</span></label>
            <textarea v-model="form.contenu" class="textarea textarea-bordered" rows="4" required placeholder="Affirmation a verifier..." />
          </div>

          <div class="form-control">
            <label class="label"><span class="label-text">Source originale (URL)</span></label>
            <input v-model="form.source_originale" type="url" class="input input-bordered" placeholder="https://...">
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Verdict</span></label>
              <select v-model="form.verdict" class="select select-bordered">
                <option value="vrai">Vrai</option>
                <option value="faux">Faux</option>
                <option value="partiellement_vrai">Partiellement vrai</option>
                <option value="trompeur">Trompeur</option>
                <option value="non_verifie">Non verifie</option>
              </select>
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Etat</span></label>
              <select v-model="form.etat" class="select select-bordered">
                <option value="publie">Publie</option>
                <option value="brouillon">Brouillon</option>
              </select>
            </div>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Territoire (UUID)</span></label>
              <input v-model="form.pays_id" type="text" class="input input-bordered" placeholder="ID du territoire">
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Couleur de fond</span></label>
              <div class="flex items-center gap-3">
                <input v-model="form.couleur_fond" type="color" class="w-12 h-10 rounded cursor-pointer border border-base-300">
                <input v-model="form.couleur_fond" type="text" class="input input-bordered input-sm w-32" placeholder="#FFFFFF">
              </div>
            </div>
          </div>

          <div class="flex justify-end gap-2 pt-4">
            <NuxtLink to="/admin/factcheck" class="btn btn-ghost">Annuler</NuxtLink>
            <button type="submit" class="btn btn-primary" :class="{ loading }" :disabled="loading">
              <font-awesome-icon v-if="!loading" icon="plus" class="mr-1" /> Creer
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>
