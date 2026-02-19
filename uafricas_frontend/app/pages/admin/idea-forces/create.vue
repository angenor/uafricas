<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { creer, loading, error } = useAdminIdeaForces()
const router = useRouter()

const form = reactive({
  titre: '',
  categorie_proposition: 'amelioration_gouvernance',
  categorie_proposition_detail: '',
  urgence: 'faible',
  pays_id: '',
  region: '',
  ville_quartier_zone: '',
  description_generale: '',
  details_proposition: '',
  plan_implementation: '',
  ressources_necessaires: '',
  impact_attendu: '',
  etat: 'en_attente',
})

const erreurLocale = ref<string | null>(null)

const soumettre = async () => {
  erreurLocale.value = null
  if (!form.titre.trim()) {
    erreurLocale.value = 'Le titre est requis'
    return
  }
  if (!form.description_generale.trim()) {
    erreurLocale.value = 'La description generale est requise'
    return
  }
  if (!form.details_proposition.trim()) {
    erreurLocale.value = 'Les details de la proposition sont requis'
    return
  }
  try {
    const body: any = {
      titre: form.titre.trim(),
      categorie_proposition: form.categorie_proposition,
      urgence: form.urgence,
      etat: form.etat,
      description_generale: form.description_generale.trim(),
      details_proposition: form.details_proposition.trim(),
    }
    if (form.categorie_proposition === 'autre' && form.categorie_proposition_detail.trim()) {
      body.categorie_proposition_detail = form.categorie_proposition_detail.trim()
    }
    if (form.pays_id.trim()) body.pays_id = form.pays_id.trim()
    if (form.region.trim()) body.region = form.region.trim()
    if (form.ville_quartier_zone.trim()) body.ville_quartier_zone = form.ville_quartier_zone.trim()
    if (form.plan_implementation.trim()) body.plan_implementation = form.plan_implementation.trim()
    if (form.ressources_necessaires.trim()) body.ressources_necessaires = form.ressources_necessaires.trim()
    if (form.impact_attendu.trim()) body.impact_attendu = form.impact_attendu.trim()
    await creer(body)
    router.push('/admin/idea-forces')
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de la creation'
  }
}
</script>

<template>
  <div>
    <AdminPageHeader titre="Nouvelle idee force" sous-titre="Ajouter une proposition citoyenne positive">
      <template #actions>
        <NuxtLink to="/admin/idea-forces" class="btn btn-ghost btn-sm">
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
              <label class="label"><span class="label-text">Titre *</span></label>
              <input v-model="form.titre" type="text" class="input input-bordered" required placeholder="Titre de la proposition">
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Categorie *</span></label>
              <select v-model="form.categorie_proposition" class="select select-bordered" required>
                <option value="amelioration_gouvernance">Gouvernance</option>
                <option value="education_formation">Education</option>
                <option value="sante_publique">Sante</option>
                <option value="emploi_jeunes">Emploi</option>
                <option value="environnement">Environnement</option>
                <option value="transport">Transport</option>
                <option value="autre">Autre</option>
              </select>
            </div>
          </div>

          <div v-if="form.categorie_proposition === 'autre'" class="form-control">
            <label class="label"><span class="label-text">Precision de la categorie</span></label>
            <input v-model="form.categorie_proposition_detail" type="text" class="input input-bordered" placeholder="Precisez la categorie...">
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Urgence</span></label>
              <select v-model="form.urgence" class="select select-bordered">
                <option value="faible">Faible</option>
                <option value="elevee">Elevee</option>
                <option value="critique">Critique</option>
              </select>
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Etat</span></label>
              <select v-model="form.etat" class="select select-bordered">
                <option value="en_attente">En attente</option>
                <option value="publie">Publie</option>
              </select>
            </div>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Pays (UUID)</span></label>
              <input v-model="form.pays_id" type="text" class="input input-bordered" placeholder="ID du pays">
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Region</span></label>
              <input v-model="form.region" type="text" class="input input-bordered" placeholder="Ex: Littoral, Dakar...">
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Ville / Quartier / Zone</span></label>
              <input v-model="form.ville_quartier_zone" type="text" class="input input-bordered" placeholder="Ex: Douala, Bonaberi...">
            </div>
          </div>

          <div class="form-control">
            <label class="label"><span class="label-text">Description generale *</span></label>
            <textarea v-model="form.description_generale" class="textarea textarea-bordered" rows="4" required placeholder="Description generale de la proposition..." />
          </div>

          <div class="form-control">
            <label class="label"><span class="label-text">Details de la proposition *</span></label>
            <textarea v-model="form.details_proposition" class="textarea textarea-bordered" rows="4" required placeholder="Details techniques et pratiques de la proposition..." />
          </div>

          <div class="form-control">
            <label class="label"><span class="label-text">Plan d'implementation</span></label>
            <textarea v-model="form.plan_implementation" class="textarea textarea-bordered" rows="3" placeholder="Etapes de mise en oeuvre..." />
          </div>

          <div class="form-control">
            <label class="label"><span class="label-text">Ressources necessaires</span></label>
            <textarea v-model="form.ressources_necessaires" class="textarea textarea-bordered" rows="3" placeholder="Budget, materiels, ressources humaines..." />
          </div>

          <div class="form-control">
            <label class="label"><span class="label-text">Impact attendu</span></label>
            <textarea v-model="form.impact_attendu" class="textarea textarea-bordered" rows="3" placeholder="Impact social, economique, environnemental attendu..." />
          </div>

          <div class="flex justify-end gap-2 pt-4">
            <NuxtLink to="/admin/idea-forces" class="btn btn-ghost">Annuler</NuxtLink>
            <button type="submit" class="btn btn-primary" :class="{ loading }" :disabled="loading">
              <font-awesome-icon v-if="!loading" icon="plus" class="mr-1" /> Creer
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>
