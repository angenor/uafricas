<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { creer, loading, error } = useAdminProgrammes()
const { listerTousDomaines } = useAdminDomaines()
const { listerPays } = useCentresCulturels()
const router = useRouter()

// Sélecteurs de référentiel (audit #20 : fini les UUID en saisie libre)
const domainesListe = ref<{ id: string, nom: string }[]>([])
const paysListe = ref<{ id: string, nom: string }[]>([])

onMounted(async () => {
  const [domaines, pays] = await Promise.all([listerTousDomaines(), listerPays()])
  domainesListe.value = domaines.map(d => ({ id: d.id, nom: d.nom }))
  paysListe.value = pays
})

const form = reactive({
  titre: '',
  description: '',
  pays_id: '',
  ville: '',
  adresse: '',
  prise_en_charge_billet: false,
  prise_en_charge_hebergement: false,
  prise_en_charge_subsistance: false,
  prise_en_charge_details: '',
  duree: '',
  domaine_id: '',
  date_debut: '',
  date_fin: '',
  nombre_places: null as number | null,
  prerequis: '',
  langues_requises: [] as string[],
})

const langueInput = ref('')
const erreurLocale = ref<string | null>(null)

const ajouterLangue = () => {
  const l = langueInput.value.trim()
  if (l && !form.langues_requises.includes(l)) {
    form.langues_requises.push(l)
    langueInput.value = ''
  }
}

const retirerLangue = (index: number) => {
  form.langues_requises.splice(index, 1)
}

const durees = [
  { label: '1 semaine', value: '1_semaine' },
  { label: '2 semaines', value: '2_semaines' },
  { label: '3 semaines', value: '3_semaines' },
  { label: '6 semaines', value: '6_semaines' },
  { label: '1 mois', value: '1_mois' },
  { label: '2 mois', value: '2_mois' },
  { label: '3 mois', value: '3_mois' },
  { label: '6 mois', value: '6_mois' },
  { label: '1 an', value: '1_an' },
]

const soumettre = async () => {
  erreurLocale.value = null
  if (!form.titre.trim()) {
    erreurLocale.value = 'Le titre du programme est requis'
    return
  }
  try {
    const body: any = { titre: form.titre.trim() }
    if (form.description.trim()) body.description = form.description.trim()
    if (form.pays_id) body.pays_id = form.pays_id
    if (form.ville.trim()) body.ville = form.ville.trim()
    if (form.adresse.trim()) body.adresse = form.adresse.trim()
    body.prise_en_charge_billet = form.prise_en_charge_billet
    body.prise_en_charge_hebergement = form.prise_en_charge_hebergement
    body.prise_en_charge_subsistance = form.prise_en_charge_subsistance
    if (form.prise_en_charge_details.trim()) body.prise_en_charge_details = form.prise_en_charge_details.trim()
    if (form.duree) body.duree = form.duree
    if (form.domaine_id) body.domaine_id = form.domaine_id
    if (form.date_debut) body.date_debut = form.date_debut
    if (form.date_fin) body.date_fin = form.date_fin
    if (form.nombre_places !== null) body.nombre_places = form.nombre_places
    if (form.prerequis.trim()) body.prerequis = form.prerequis.trim()
    if (form.langues_requises.length) body.langues_requises = form.langues_requises
    await creer(body)
    router.push('/admin/programmes')
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de la creation'
  }
}
</script>

<template>
  <div>
    <AdminPageHeader titre="Nouveau programme" sous-titre="Creer un programme d'echange">
      <template #actions>
        <NuxtLink to="/admin/programmes" class="btn btn-ghost btn-sm">
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
              <label class="label"><span class="label-text">Titre du programme *</span></label>
              <input v-model="form.titre" type="text" class="input input-bordered" required placeholder="Ex: Programme d'echange universitaire Cameroun-Senegal">
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Description</span></label>
              <textarea v-model="form.description" class="textarea textarea-bordered h-32" placeholder="Description detaillee du programme, objectifs, contexte..." />
            </div>
          </div>

          <!-- Localisation & domaine -->
          <div class="space-y-4">
            <h3 class="text-lg font-semibold border-b pb-2">Localisation & domaine</h3>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Territoire de destination</span></label>
                <select v-model="form.pays_id" class="select select-bordered">
                  <option value="">Sélectionner un territoire</option>
                  <option v-for="p in paysListe" :key="p.id" :value="p.id">{{ p.nom }}</option>
                </select>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Domaine</span></label>
                <select v-model="form.domaine_id" class="select select-bordered">
                  <option value="">Sélectionner un domaine</option>
                  <option v-for="d in domainesListe" :key="d.id" :value="d.id">{{ d.nom }}</option>
                </select>
              </div>
            </div>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Ville</span></label>
                <input v-model="form.ville" type="text" class="input input-bordered" placeholder="Ex: Dakar">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Adresse</span></label>
                <input v-model="form.adresse" type="text" class="input input-bordered">
              </div>
            </div>
          </div>

          <!-- Dates & duree -->
          <div class="space-y-4">
            <h3 class="text-lg font-semibold border-b pb-2">Dates & capacite</h3>
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Duree</span></label>
                <select v-model="form.duree" class="select select-bordered">
                  <option value="">Non specifie</option>
                  <option v-for="d in durees" :key="d.value" :value="d.value">{{ d.label }}</option>
                </select>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Date de debut</span></label>
                <input v-model="form.date_debut" type="date" class="input input-bordered">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Date de fin</span></label>
                <input v-model="form.date_fin" type="date" class="input input-bordered">
              </div>
            </div>
            <div class="form-control w-48">
              <label class="label"><span class="label-text">Nombre de places</span></label>
              <input v-model.number="form.nombre_places" type="number" min="1" class="input input-bordered">
            </div>
          </div>

          <!-- Couverture -->
          <div class="space-y-4">
            <h3 class="text-lg font-semibold border-b pb-2">Prise en charge</h3>
            <div class="flex flex-wrap gap-6">
              <label class="label cursor-pointer gap-2">
                <input v-model="form.prise_en_charge_billet" type="checkbox" class="checkbox checkbox-primary" />
                <span class="label-text">Billet d'avion</span>
              </label>
              <label class="label cursor-pointer gap-2">
                <input v-model="form.prise_en_charge_hebergement" type="checkbox" class="checkbox checkbox-primary" />
                <span class="label-text">Hebergement</span>
              </label>
              <label class="label cursor-pointer gap-2">
                <input v-model="form.prise_en_charge_subsistance" type="checkbox" class="checkbox checkbox-primary" />
                <span class="label-text">Subsistance</span>
              </label>
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Details de la prise en charge</span></label>
              <textarea v-model="form.prise_en_charge_details" class="textarea textarea-bordered" placeholder="Precisions sur la couverture financiere..." />
            </div>
          </div>

          <!-- Prerequis & langues -->
          <div class="space-y-4">
            <h3 class="text-lg font-semibold border-b pb-2">Prerequis</h3>
            <div class="form-control">
              <label class="label"><span class="label-text">Prerequis</span></label>
              <textarea v-model="form.prerequis" class="textarea textarea-bordered" placeholder="Diplome minimum, experience requise..." />
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Langues requises</span></label>
              <div class="flex gap-2">
                <input v-model="langueInput" type="text" class="input input-bordered flex-1" placeholder="Ex: Francais" @keydown.enter.prevent="ajouterLangue">
                <button type="button" class="btn btn-outline" @click="ajouterLangue">Ajouter</button>
              </div>
              <div v-if="form.langues_requises.length" class="flex flex-wrap gap-2 mt-2">
                <span v-for="(l, i) in form.langues_requises" :key="i" class="badge badge-primary gap-1">
                  {{ l }}
                  <button type="button" class="btn btn-ghost btn-xs p-0" @click="retirerLangue(i)">&times;</button>
                </span>
              </div>
            </div>
          </div>

          <div class="flex justify-end gap-2 pt-4">
            <NuxtLink to="/admin/programmes" class="btn btn-ghost">Annuler</NuxtLink>
            <button type="submit" class="btn btn-primary" :class="{ loading }" :disabled="loading">
              <font-awesome-icon v-if="!loading" icon="plus" class="mr-1" /> Creer
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>
