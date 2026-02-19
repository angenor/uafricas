<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { creer, loading, error } = useAdminProjets()
const router = useRouter()

const form = reactive({
  titre: '',
  description: '',
  objectifs: '',
  nom_organisation: '',
  description_organisation: '',
  site_web: '',
  pays_id: '',
  ville: '',
  contact_email: '',
  contact_telephone: '',
  cout_total: null as number | null,
  devise: 'XOF',
  duree_mois: null as number | null,
  date_commencement_souhaitee: '',
  resultats_attendus: '',
  activites_programmees: '',
  echeanciers: '',
  contribution_autonomisation: '',
  difficultes_risques: '',
})

const erreurLocale = ref<string | null>(null)

const soumettre = async () => {
  erreurLocale.value = null
  if (!form.titre.trim()) {
    erreurLocale.value = 'Le titre est requis'
    return
  }
  if (!form.description.trim()) {
    erreurLocale.value = 'La description est requise'
    return
  }
  if (!form.objectifs.trim()) {
    erreurLocale.value = 'Les objectifs sont requis'
    return
  }
  try {
    const body: any = {
      titre: form.titre.trim(),
      description: form.description.trim(),
      objectifs: form.objectifs.trim(),
    }
    if (form.nom_organisation.trim()) body.nom_organisation = form.nom_organisation.trim()
    if (form.description_organisation.trim()) body.description_organisation = form.description_organisation.trim()
    if (form.site_web.trim()) body.site_web = form.site_web.trim()
    if (form.pays_id) body.pays_id = form.pays_id
    if (form.ville.trim()) body.ville = form.ville.trim()
    if (form.contact_email.trim()) body.contact_email = form.contact_email.trim()
    if (form.contact_telephone.trim()) body.contact_telephone = form.contact_telephone.trim()
    if (form.cout_total !== null) body.cout_total = form.cout_total
    body.devise = form.devise
    if (form.duree_mois !== null) body.duree_mois = form.duree_mois
    if (form.date_commencement_souhaitee) body.date_commencement_souhaitee = form.date_commencement_souhaitee
    if (form.resultats_attendus.trim()) body.resultats_attendus = form.resultats_attendus.trim()
    if (form.activites_programmees.trim()) body.activites_programmees = form.activites_programmees.trim()
    if (form.echeanciers.trim()) body.echeanciers = form.echeanciers.trim()
    if (form.contribution_autonomisation.trim()) body.contribution_autonomisation = form.contribution_autonomisation.trim()
    if (form.difficultes_risques.trim()) body.difficultes_risques = form.difficultes_risques.trim()
    await creer(body)
    router.push('/admin/projets')
  }
  catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de la creation'
  }
}
</script>

<template>
  <div>
    <AdminPageHeader titre="Nouveau projet" sous-titre="Soumettre un projet a financer">
      <template #actions>
        <NuxtLink to="/admin/projets" class="btn btn-ghost btn-sm">
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
          <!-- Section : Informations de base -->
          <h3 class="font-semibold text-lg border-b pb-2">Informations de base</h3>

          <div class="form-control">
            <label class="label"><span class="label-text">Titre du projet *</span></label>
            <input v-model="form.titre" type="text" class="input input-bordered" required>
          </div>

          <div class="form-control">
            <label class="label"><span class="label-text">Description *</span></label>
            <textarea v-model="form.description" class="textarea textarea-bordered" rows="4" required />
          </div>

          <div class="form-control">
            <label class="label"><span class="label-text">Objectifs *</span></label>
            <textarea v-model="form.objectifs" class="textarea textarea-bordered" rows="3" required />
          </div>

          <!-- Section : Organisation -->
          <h3 class="font-semibold text-lg border-b pb-2">Organisation porteuse</h3>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Nom de l'organisation</span></label>
              <input v-model="form.nom_organisation" type="text" class="input input-bordered">
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Site web</span></label>
              <input v-model="form.site_web" type="url" class="input input-bordered" placeholder="https://...">
            </div>
          </div>

          <div class="form-control">
            <label class="label"><span class="label-text">Description de l'organisation</span></label>
            <textarea v-model="form.description_organisation" class="textarea textarea-bordered" rows="2" />
          </div>

          <!-- Section : Localisation & Contact -->
          <h3 class="font-semibold text-lg border-b pb-2">Localisation & Contact</h3>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Ville</span></label>
              <input v-model="form.ville" type="text" class="input input-bordered">
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Email de contact</span></label>
              <input v-model="form.contact_email" type="email" class="input input-bordered">
            </div>
          </div>

          <div class="form-control">
            <label class="label"><span class="label-text">Telephone de contact</span></label>
            <input v-model="form.contact_telephone" type="tel" class="input input-bordered" class-name="w-1/2">
          </div>

          <!-- Section : Budget & Calendrier -->
          <h3 class="font-semibold text-lg border-b pb-2">Budget & Calendrier</h3>

          <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Cout total</span></label>
              <input v-model.number="form.cout_total" type="number" step="0.01" min="0" class="input input-bordered">
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Devise</span></label>
              <select v-model="form.devise" class="select select-bordered">
                <option value="XOF">XOF (Franc CFA)</option>
                <option value="EUR">EUR (Euro)</option>
                <option value="USD">USD (Dollar US)</option>
              </select>
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Duree (mois)</span></label>
              <input v-model.number="form.duree_mois" type="number" min="1" class="input input-bordered">
            </div>
          </div>

          <div class="form-control">
            <label class="label"><span class="label-text">Date de commencement souhaitee</span></label>
            <input v-model="form.date_commencement_souhaitee" type="date" class="input input-bordered w-1/2">
          </div>

          <!-- Section : Details du projet -->
          <h3 class="font-semibold text-lg border-b pb-2">Details du projet</h3>

          <div class="form-control">
            <label class="label"><span class="label-text">Resultats attendus</span></label>
            <textarea v-model="form.resultats_attendus" class="textarea textarea-bordered" rows="3" />
          </div>

          <div class="form-control">
            <label class="label"><span class="label-text">Activites programmees</span></label>
            <textarea v-model="form.activites_programmees" class="textarea textarea-bordered" rows="3" />
          </div>

          <div class="form-control">
            <label class="label"><span class="label-text">Echeanciers</span></label>
            <textarea v-model="form.echeanciers" class="textarea textarea-bordered" rows="2" />
          </div>

          <div class="form-control">
            <label class="label"><span class="label-text">Contribution a l'autonomisation</span></label>
            <textarea v-model="form.contribution_autonomisation" class="textarea textarea-bordered" rows="2" />
          </div>

          <div class="form-control">
            <label class="label"><span class="label-text">Difficultes et risques identifies</span></label>
            <textarea v-model="form.difficultes_risques" class="textarea textarea-bordered" rows="2" />
          </div>

          <div class="flex justify-end gap-2 pt-4">
            <NuxtLink to="/admin/projets" class="btn btn-ghost">Annuler</NuxtLink>
            <button type="submit" class="btn btn-primary" :class="{ loading }" :disabled="loading">
              <font-awesome-icon v-if="!loading" icon="plus" class="mr-1" /> Soumettre
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>
