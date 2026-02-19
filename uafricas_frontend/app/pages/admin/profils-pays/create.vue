<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { creer, loading, error } = useAdminProfilsPays()
const router = useRouter()

const form = reactive({
  pays_id: '',
  slogan: '',
  superficie_km2: null as number | null,
  population: null as number | null,
  biographie: '',
  contexte: '',
  contexte_historique: '',
  image_couverture_url: '',
  image_drapeau_url: '',
  image_embleme_url: '',
  image_devise_url: '',
  hymne_national: '',
  langue_officielle: '',
  langues_populaires: '',
  monnaie: '',
  fuseau_horaire: '',
})

const erreurLocale = ref<string | null>(null)

const soumettre = async () => {
  erreurLocale.value = null
  if (!form.pays_id.trim()) {
    erreurLocale.value = 'L\'identifiant du pays est requis'
    return
  }
  try {
    const body: any = {
      pays_id: form.pays_id.trim(),
    }
    if (form.slogan.trim()) body.slogan = form.slogan.trim()
    if (form.superficie_km2 !== null) body.superficie_km2 = form.superficie_km2
    if (form.population !== null) body.population = form.population
    if (form.biographie.trim()) body.biographie = form.biographie.trim()
    if (form.contexte.trim()) body.contexte = form.contexte.trim()
    if (form.contexte_historique.trim()) body.contexte_historique = form.contexte_historique.trim()
    if (form.image_couverture_url.trim()) body.image_couverture_url = form.image_couverture_url.trim()
    if (form.image_drapeau_url.trim()) body.image_drapeau_url = form.image_drapeau_url.trim()
    if (form.image_embleme_url.trim()) body.image_embleme_url = form.image_embleme_url.trim()
    if (form.image_devise_url.trim()) body.image_devise_url = form.image_devise_url.trim()
    if (form.hymne_national.trim()) body.hymne_national = form.hymne_national.trim()
    if (form.langue_officielle.trim()) body.langue_officielle = form.langue_officielle.trim()
    if (form.langues_populaires.trim()) body.langues_populaires = form.langues_populaires.trim()
    if (form.monnaie.trim()) body.monnaie = form.monnaie.trim()
    if (form.fuseau_horaire.trim()) body.fuseau_horaire = form.fuseau_horaire.trim()

    await creer(body)
    router.push('/admin/profils-pays')
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de la creation'
  }
}
</script>

<template>
  <div>
    <AdminPageHeader titre="Nouveau profil pays" sous-titre="Creer un profil pays">
      <template #actions>
        <NuxtLink to="/admin/profils-pays" class="btn btn-ghost btn-sm">
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
          <!-- Pays -->
          <div class="space-y-4">
            <h3 class="text-lg font-semibold border-b pb-2">Pays</h3>
            <div class="form-control">
              <label class="label"><span class="label-text">Identifiant du pays (UUID) *</span></label>
              <input v-model="form.pays_id" type="text" class="input input-bordered" required placeholder="Ex: 550e8400-e29b-41d4-a716-446655440000">
            </div>
          </div>

          <!-- Informations generales -->
          <div class="space-y-4">
            <h3 class="text-lg font-semibold border-b pb-2">Informations generales</h3>
            <div class="form-control">
              <label class="label"><span class="label-text">Slogan</span></label>
              <input v-model="form.slogan" type="text" class="input input-bordered" placeholder="Ex: Unite, Travail, Progres">
            </div>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Population</span></label>
                <input v-model.number="form.population" type="number" min="0" class="input input-bordered" placeholder="Ex: 27000000">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Superficie (km2)</span></label>
                <input v-model.number="form.superficie_km2" type="number" min="0" class="input input-bordered" placeholder="Ex: 322463">
              </div>
            </div>
          </div>

          <!-- Description -->
          <div class="space-y-4">
            <h3 class="text-lg font-semibold border-b pb-2">Description</h3>
            <div class="form-control">
              <label class="label"><span class="label-text">Biographie</span></label>
              <textarea v-model="form.biographie" class="textarea textarea-bordered h-32" placeholder="Presentation generale du pays..." />
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Contexte</span></label>
              <textarea v-model="form.contexte" class="textarea textarea-bordered h-24" placeholder="Contexte actuel du pays..." />
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Contexte historique</span></label>
              <textarea v-model="form.contexte_historique" class="textarea textarea-bordered h-24" placeholder="Histoire et contexte historique du pays..." />
            </div>
          </div>

          <!-- Images -->
          <div class="space-y-4">
            <h3 class="text-lg font-semibold border-b pb-2">Images</h3>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">URL de l'image de couverture</span></label>
                <input v-model="form.image_couverture_url" type="url" class="input input-bordered" placeholder="https://exemple.com/couverture.jpg">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">URL de l'image du drapeau</span></label>
                <input v-model="form.image_drapeau_url" type="url" class="input input-bordered" placeholder="https://exemple.com/drapeau.jpg">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">URL de l'image de l'embleme</span></label>
                <input v-model="form.image_embleme_url" type="url" class="input input-bordered" placeholder="https://exemple.com/embleme.jpg">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">URL de l'image de la devise</span></label>
                <input v-model="form.image_devise_url" type="url" class="input input-bordered" placeholder="https://exemple.com/devise.jpg">
              </div>
            </div>
          </div>

          <!-- Culture & symboles -->
          <div class="space-y-4">
            <h3 class="text-lg font-semibold border-b pb-2">Culture & symboles</h3>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Hymne national</span></label>
                <input v-model="form.hymne_national" type="text" class="input input-bordered" placeholder="Ex: L'Abidjanaise">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Langue officielle</span></label>
                <input v-model="form.langue_officielle" type="text" class="input input-bordered" placeholder="Ex: Francais">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Langues populaires</span></label>
                <input v-model="form.langues_populaires" type="text" class="input input-bordered" placeholder="Ex: Dioula, Baoule, Bete">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Monnaie</span></label>
                <input v-model="form.monnaie" type="text" class="input input-bordered" placeholder="Ex: Franc CFA (XOF)">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Fuseau horaire</span></label>
                <input v-model="form.fuseau_horaire" type="text" class="input input-bordered" placeholder="Ex: GMT+0">
              </div>
            </div>
          </div>

          <div class="flex justify-end gap-2 pt-4">
            <NuxtLink to="/admin/profils-pays" class="btn btn-ghost">Annuler</NuxtLink>
            <button type="submit" class="btn btn-primary" :class="{ loading }" :disabled="loading">
              <font-awesome-icon v-if="!loading" icon="plus" class="mr-1" /> Creer
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>
