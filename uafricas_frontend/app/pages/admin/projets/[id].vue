<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const route = useRoute()
const id = route.params.id as string
const { projetDetail, chargerDetail, modifier, changerEtat, ajouterDocument, retirerDocument, loading, error } = useAdminProjets()

const ongletActif = ref('infos')
const saving = ref(false)
const erreurLocale = ref<string | null>(null)
const successMsg = ref<string | null>(null)

const etatProjetConfig: Record<string, { label: string; class: string }> = {
  soumis: { label: 'Soumis', class: 'badge-info' },
  en_revue: { label: 'En revue', class: 'badge-warning' },
  approuve: { label: 'Approuve', class: 'badge-success' },
  en_cours: { label: 'En cours', class: 'badge-primary' },
  termine: { label: 'Termine', class: 'badge-neutral' },
  suspendu: { label: 'Suspendu', class: 'badge-warning' },
  rejete: { label: 'Rejete', class: 'badge-error' },
}

const form = reactive({
  titre: '',
  description: '',
  objectifs: '',
  nom_organisation: '',
  description_organisation: '',
  site_web: '',
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

const docForm = reactive({
  nom: '',
  url: '',
  type_mime: '',
})

const charger = async () => {
  await chargerDetail(id)
  if (projetDetail.value) {
    const p = projetDetail.value
    form.titre = p.titre
    form.description = p.description
    form.objectifs = p.objectifs
    form.nom_organisation = p.nom_organisation || ''
    form.description_organisation = p.description_organisation || ''
    form.site_web = p.site_web || ''
    form.ville = p.ville || ''
    form.contact_email = p.contact_email || ''
    form.contact_telephone = p.contact_telephone || ''
    form.cout_total = p.cout_total
    form.devise = p.devise || 'XOF'
    form.duree_mois = p.duree_mois
    form.date_commencement_souhaitee = p.date_commencement_souhaitee || ''
    form.resultats_attendus = p.resultats_attendus || ''
    form.activites_programmees = p.activites_programmees || ''
    form.echeanciers = p.echeanciers || ''
    form.contribution_autonomisation = p.contribution_autonomisation || ''
    form.difficultes_risques = p.difficultes_risques || ''
  }
}

const sauvegarder = async () => {
  saving.value = true
  erreurLocale.value = null
  successMsg.value = null
  try {
    const body: any = {}
    if (form.titre.trim()) body.titre = form.titre.trim()
    if (form.description.trim()) body.description = form.description.trim()
    if (form.objectifs.trim()) body.objectifs = form.objectifs.trim()
    if (form.nom_organisation.trim()) body.nom_organisation = form.nom_organisation.trim()
    if (form.description_organisation.trim()) body.description_organisation = form.description_organisation.trim()
    if (form.site_web.trim()) body.site_web = form.site_web.trim()
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
    await modifier(id, body)
    successMsg.value = 'Projet mis a jour'
    setTimeout(() => { successMsg.value = null }, 3000)
  }
  catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  }
  finally { saving.value = false }
}

const modererEtat = async (nouvelEtat: string) => {
  saving.value = true
  erreurLocale.value = null
  try {
    await changerEtat(id, nouvelEtat)
    await chargerDetail(id)
    successMsg.value = `Etat change en "${etatProjetConfig[nouvelEtat]?.label || nouvelEtat}"`
    setTimeout(() => { successMsg.value = null }, 3000)
  }
  catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  }
  finally { saving.value = false }
}

const ajouterNouveauDocument = async () => {
  if (!docForm.nom.trim() || !docForm.url.trim()) return
  try {
    await ajouterDocument(id, {
      nom: docForm.nom.trim(),
      url: docForm.url.trim(),
      type_mime: docForm.type_mime || undefined,
    })
    docForm.nom = ''
    docForm.url = ''
    docForm.type_mime = ''
    await chargerDetail(id)
  }
  catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  }
}

const supprimerDocument = async (docId: string) => {
  try {
    await retirerDocument(id, docId)
    await chargerDetail(id)
  }
  catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  }
}

onMounted(() => charger())
</script>

<template>
  <div>
    <AdminPageHeader :titre="projetDetail?.titre || 'Chargement...'" sous-titre="Modifier le projet">
      <template #actions>
        <NuxtLink to="/admin/projets" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Retour
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div v-if="loading && !projetDetail" class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <div v-else-if="projetDetail">
      <div v-if="erreurLocale || error" class="alert alert-error mb-4">
        <font-awesome-icon icon="circle-exclamation" />
        <span>{{ erreurLocale || error }}</span>
      </div>
      <div v-if="successMsg" class="alert alert-success mb-4">
        <font-awesome-icon icon="circle-check" />
        <span>{{ successMsg }}</span>
      </div>

      <!-- Onglets -->
      <div class="tabs tabs-bordered mb-4">
        <a class="tab" :class="{ 'tab-active': ongletActif === 'infos' }" @click="ongletActif = 'infos'">Infos</a>
        <a class="tab" :class="{ 'tab-active': ongletActif === 'documents' }" @click="ongletActif = 'documents'">
          Documents ({{ projetDetail.documents?.length || 0 }})
        </a>
      </div>

      <!-- Onglet Infos -->
      <div v-if="ongletActif === 'infos'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <!-- Workflow approbation -->
          <div class="flex flex-wrap items-center gap-2 mb-4 p-3 bg-base-200 rounded-lg">
            <span class="font-semibold text-sm">Etat actuel :</span>
            <span class="badge" :class="etatProjetConfig[projetDetail.etat]?.class || 'badge-neutral'">
              {{ etatProjetConfig[projetDetail.etat]?.label || projetDetail.etat }}
            </span>
            <div v-if="projetDetail.traite_par_nom" class="text-xs text-base-content/50">
              Traite par {{ projetDetail.traite_par_prenom }} {{ projetDetail.traite_par_nom }}
            </div>
            <div class="ml-auto flex gap-1 flex-wrap">
              <button
                v-if="projetDetail.etat === 'soumis'"
                class="btn btn-warning btn-xs"
                :disabled="saving"
                @click="modererEtat('en_revue')"
              >
                Mettre en revue
              </button>
              <button
                v-if="['soumis', 'en_revue'].includes(projetDetail.etat)"
                class="btn btn-success btn-xs"
                :disabled="saving"
                @click="modererEtat('approuve')"
              >
                Approuver
              </button>
              <button
                v-if="projetDetail.etat === 'approuve'"
                class="btn btn-primary btn-xs"
                :disabled="saving"
                @click="modererEtat('en_cours')"
              >
                Demarrer
              </button>
              <button
                v-if="projetDetail.etat === 'en_cours'"
                class="btn btn-neutral btn-xs"
                :disabled="saving"
                @click="modererEtat('termine')"
              >
                Terminer
              </button>
              <button
                v-if="!['rejete', 'termine'].includes(projetDetail.etat)"
                class="btn btn-error btn-xs"
                :disabled="saving"
                @click="modererEtat('rejete')"
              >
                Rejeter
              </button>
              <button
                v-if="!['suspendu', 'termine', 'rejete'].includes(projetDetail.etat)"
                class="btn btn-warning btn-xs"
                :disabled="saving"
                @click="modererEtat('suspendu')"
              >
                Suspendre
              </button>
            </div>
          </div>

          <form @submit.prevent="sauvegarder" class="space-y-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Titre *</span></label>
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

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Organisation</span></label>
                <input v-model="form.nom_organisation" type="text" class="input input-bordered">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Site web</span></label>
                <input v-model="form.site_web" type="url" class="input input-bordered">
              </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Budget</span></label>
                <input v-model.number="form.cout_total" type="number" step="0.01" min="0" class="input input-bordered">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Devise</span></label>
                <select v-model="form.devise" class="select select-bordered">
                  <option value="XOF">XOF</option>
                  <option value="EUR">EUR</option>
                  <option value="USD">USD</option>
                </select>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Duree (mois)</span></label>
                <input v-model.number="form.duree_mois" type="number" min="1" class="input input-bordered">
              </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Date commencement</span></label>
                <input v-model="form.date_commencement_souhaitee" type="date" class="input input-bordered">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Ville</span></label>
                <input v-model="form.ville" type="text" class="input input-bordered">
              </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Email contact</span></label>
                <input v-model="form.contact_email" type="email" class="input input-bordered">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Telephone contact</span></label>
                <input v-model="form.contact_telephone" type="tel" class="input input-bordered">
              </div>
            </div>

            <div class="form-control">
              <label class="label"><span class="label-text">Resultats attendus</span></label>
              <textarea v-model="form.resultats_attendus" class="textarea textarea-bordered" rows="2" />
            </div>

            <div class="form-control">
              <label class="label"><span class="label-text">Activites programmees</span></label>
              <textarea v-model="form.activites_programmees" class="textarea textarea-bordered" rows="2" />
            </div>

            <div class="form-control">
              <label class="label"><span class="label-text">Echeanciers</span></label>
              <textarea v-model="form.echeanciers" class="textarea textarea-bordered" rows="2" />
            </div>

            <div class="form-control">
              <label class="label"><span class="label-text">Difficultes et risques</span></label>
              <textarea v-model="form.difficultes_risques" class="textarea textarea-bordered" rows="2" />
            </div>

            <div class="flex items-center justify-between pt-4">
              <div class="text-sm text-base-content/50">
                Auteur : {{ projetDetail.auteur_prenom }} {{ projetDetail.auteur_nom }} |
                Territoire : {{ projetDetail.pays_nom || 'Non defini' }}
              </div>
              <button type="submit" class="btn btn-primary" :class="{ loading: saving }" :disabled="saving">
                <font-awesome-icon v-if="!saving" icon="floppy-disk" class="mr-1" /> Enregistrer
              </button>
            </div>
          </form>
        </div>
      </div>

      <!-- Onglet Documents -->
      <div v-if="ongletActif === 'documents'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <!-- Ajouter document -->
          <div class="flex flex-wrap gap-2 mb-4">
            <input v-model="docForm.nom" type="text" class="input input-bordered input-sm flex-1" placeholder="Nom du document">
            <input v-model="docForm.url" type="text" class="input input-bordered input-sm flex-1" placeholder="URL du document">
            <input v-model="docForm.type_mime" type="text" class="input input-bordered input-sm w-36" placeholder="Type MIME">
            <button class="btn btn-primary btn-sm" @click="ajouterNouveauDocument" :disabled="!docForm.nom.trim() || !docForm.url.trim()">
              <font-awesome-icon icon="plus" class="mr-1" /> Ajouter
            </button>
          </div>

          <!-- Liste documents -->
          <div v-if="projetDetail.documents?.length" class="overflow-x-auto">
            <table class="table table-sm">
              <thead>
                <tr>
                  <th>Nom</th>
                  <th>Type</th>
                  <th>Date</th>
                  <th class="w-24">Actions</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="doc in projetDetail.documents" :key="doc.id">
                  <td>
                    <a :href="doc.url" target="_blank" class="link link-primary text-sm">
                      {{ doc.nom }}
                    </a>
                  </td>
                  <td class="text-xs text-base-content/60">{{ doc.type_mime || 'Inconnu' }}</td>
                  <td class="text-xs">{{ new Date(doc.created_at).toLocaleDateString('fr-FR') }}</td>
                  <td>
                    <div class="flex gap-1">
                      <a :href="doc.url" target="_blank" class="btn btn-ghost btn-xs">
                        <font-awesome-icon icon="download" />
                      </a>
                      <button class="btn btn-ghost btn-xs text-error" @click="supprimerDocument(doc.id)">
                        <font-awesome-icon icon="trash" />
                      </button>
                    </div>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
          <div v-else class="text-center text-base-content/50 py-8">
            Aucun document associe a ce projet
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
