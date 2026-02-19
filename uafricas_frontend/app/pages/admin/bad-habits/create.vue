<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { creer, loading, error } = useAdminBadHabits()
const router = useRouter()

const form = reactive({
  titre: '',
  categorie_probleme: 'corruption',
  categorie_probleme_detail: '',
  gravite: 'elevee',
  pays_id: '',
  region: '',
  ville_quartier_zone: '',
  description_generale: '',
  details_problematique: '',
  preuves_temoignages: '',
  solutions_proposees: '',
  publication_anonyme: false,
  geolocalisation_autorisee: false,
  longitude: null as number | null,
  latitude: null as number | null,
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
  if (!form.details_problematique.trim()) {
    erreurLocale.value = 'Les details de la problematique sont requis'
    return
  }
  try {
    const body: any = {
      titre: form.titre.trim(),
      categorie_probleme: form.categorie_probleme,
      gravite: form.gravite,
      description_generale: form.description_generale.trim(),
      details_problematique: form.details_problematique.trim(),
      publication_anonyme: form.publication_anonyme,
      geolocalisation_autorisee: form.geolocalisation_autorisee,
      etat: form.etat,
    }
    if (form.categorie_probleme === 'autre' && form.categorie_probleme_detail.trim()) {
      body.categorie_probleme_detail = form.categorie_probleme_detail.trim()
    }
    if (form.pays_id.trim()) body.pays_id = form.pays_id.trim()
    if (form.region.trim()) body.region = form.region.trim()
    if (form.ville_quartier_zone.trim()) body.ville_quartier_zone = form.ville_quartier_zone.trim()
    if (form.preuves_temoignages.trim()) body.preuves_temoignages = form.preuves_temoignages.trim()
    if (form.solutions_proposees.trim()) body.solutions_proposees = form.solutions_proposees.trim()
    if (form.geolocalisation_autorisee) {
      if (form.longitude !== null) body.longitude = form.longitude
      if (form.latitude !== null) body.latitude = form.latitude
    }
    await creer(body)
    router.push('/admin/bad-habits')
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de la creation'
  }
}
</script>

<template>
  <div>
    <AdminPageHeader titre="Nouveau signalement" sous-titre="Ajouter une mauvaise pratique">
      <template #actions>
        <NuxtLink to="/admin/bad-habits" class="btn btn-ghost btn-sm">
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
          <!-- Titre + categorie -->
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Titre *</span></label>
              <input v-model="form.titre" type="text" class="input input-bordered" required placeholder="Titre du signalement">
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Categorie du probleme *</span></label>
              <select v-model="form.categorie_probleme" class="select select-bordered" required>
                <option value="corruption">Corruption</option>
                <option value="service_public_defaillant">Service public defaillant</option>
                <option value="infrastructure_degradee">Infrastructure degradee</option>
                <option value="acces_services_limite">Acces services limite</option>
                <option value="insalubrite">Insalubrite</option>
                <option value="probleme_securite">Probleme securite</option>
                <option value="autre">Autre</option>
              </select>
            </div>
          </div>

          <!-- Detail categorie (si autre) -->
          <div v-if="form.categorie_probleme === 'autre'" class="form-control">
            <label class="label"><span class="label-text">Preciser la categorie</span></label>
            <input v-model="form.categorie_probleme_detail" type="text" class="input input-bordered" placeholder="Preciser le type de probleme...">
          </div>

          <!-- Gravite + Etat -->
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Gravite</span></label>
              <select v-model="form.gravite" class="select select-bordered">
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

          <!-- Localisation -->
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
              <input v-model="form.ville_quartier_zone" type="text" class="input input-bordered" placeholder="Ex: Douala, Medina...">
            </div>
          </div>

          <!-- Description generale -->
          <div class="form-control">
            <label class="label"><span class="label-text">Description generale *</span></label>
            <textarea v-model="form.description_generale" class="textarea textarea-bordered" rows="4" required placeholder="Decrivez la situation de maniere generale..." />
          </div>

          <!-- Details problematique -->
          <div class="form-control">
            <label class="label"><span class="label-text">Details de la problematique *</span></label>
            <textarea v-model="form.details_problematique" class="textarea textarea-bordered" rows="4" required placeholder="Detaillez la problematique specifique..." />
          </div>

          <!-- Preuves et temoignages -->
          <div class="form-control">
            <label class="label"><span class="label-text">Preuves et temoignages</span></label>
            <textarea v-model="form.preuves_temoignages" class="textarea textarea-bordered" rows="3" placeholder="Temoignages, faits verifiables..." />
          </div>

          <!-- Solutions proposees -->
          <div class="form-control">
            <label class="label"><span class="label-text">Solutions proposees</span></label>
            <textarea v-model="form.solutions_proposees" class="textarea textarea-bordered" rows="3" placeholder="Quelles solutions envisagez-vous ?" />
          </div>

          <!-- Options de publication -->
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="form-control">
              <label class="label cursor-pointer justify-start gap-3">
                <input v-model="form.publication_anonyme" type="checkbox" class="toggle toggle-primary" />
                <span class="label-text">Publication anonyme</span>
              </label>
            </div>
            <div class="form-control">
              <label class="label cursor-pointer justify-start gap-3">
                <input v-model="form.geolocalisation_autorisee" type="checkbox" class="toggle toggle-primary" />
                <span class="label-text">Geolocalisation autorisee</span>
              </label>
            </div>
          </div>

          <!-- Coordonnees GPS (si geolocalisation autorisee) -->
          <div v-if="form.geolocalisation_autorisee" class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Longitude</span></label>
              <input v-model.number="form.longitude" type="number" step="any" class="input input-bordered" placeholder="Ex: 9.7023">
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Latitude</span></label>
              <input v-model.number="form.latitude" type="number" step="any" class="input input-bordered" placeholder="Ex: 4.0511">
            </div>
          </div>

          <div class="flex justify-end gap-2 pt-4">
            <NuxtLink to="/admin/bad-habits" class="btn btn-ghost">Annuler</NuxtLink>
            <button type="submit" class="btn btn-primary" :class="{ loading }" :disabled="loading">
              <font-awesome-icon v-if="!loading" icon="plus" class="mr-1" /> Creer
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>
