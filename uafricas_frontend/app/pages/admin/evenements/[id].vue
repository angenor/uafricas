<script setup lang="ts">
import type { StatsCardData } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const route = useRoute()
const id = route.params.id as string
const {
  evenementDetail, inscriptions, inscriptionStats,
  chargerDetail, modifier, changerEtat,
  chargerInscriptions, changerStatutInscription, chargerStatsInscriptions,
  loading, error,
} = useAdminEvenements()

const saving = ref(false)
const erreurLocale = ref<string | null>(null)
const successMsg = ref<string | null>(null)
const ongletActif = ref<'infos' | 'inscriptions'>('infos')

const form = reactive({
  titre: '',
  description: '',
  type_evenement: '',
  format: 'presentiel',
  langue: 'fr',
  date_heure_debut: '',
  date_heure_fin: '',
  nombre_places: null as number | null,
  pays_id: '',
  ville: '',
  adresse: '',
  lien_en_ligne: '',
  enregistrement_url: '',
  image_couverture_url: '',
  type_organisateur: 'personnel' as 'personnel' | 'organisation',
  contact_nom: '',
  contact_email: '',
  contact_telephone: '',
  contact_site_web: '',
})

const afficherLienEnLigne = computed(() => form.format === 'en_ligne' || form.format === 'hybride')

// Etat moderation
const showEtatModal = ref(false)
const nouvelEtat = ref('')
const etatLoading = ref(false)

// Badges & labels
const etatBadge = (etat: string) => {
  const map: Record<string, string> = {
    brouillon: 'badge-warning',
    publie: 'badge-success',
    suspendu: 'badge-error',
    annule: 'badge-ghost',
  }
  return map[etat] || 'badge-neutral'
}

const etatLabel = (etat: string) => {
  const map: Record<string, string> = {
    brouillon: 'Brouillon',
    publie: 'Publie',
    suspendu: 'Suspendu',
    annule: 'Annule',
  }
  return map[etat] || etat
}

const statutBadge = (statut: string) => {
  const map: Record<string, string> = {
    inscrit: 'badge-info',
    confirme: 'badge-success',
    annule: 'badge-error',
    present: 'badge-primary',
    absent: 'badge-neutral',
  }
  return map[statut] || 'badge-neutral'
}

const statutLabel = (statut: string) => {
  const map: Record<string, string> = {
    inscrit: 'Inscrit',
    confirme: 'Confirme',
    annule: 'Annule',
    present: 'Present',
    absent: 'Absent',
  }
  return map[statut] || statut
}

// Stats cards
const statsCards = computed<StatsCardData[]>(() => {
  if (!inscriptionStats.value) return []
  const s = inscriptionStats.value
  return [
    { label: 'Total inscriptions', value: s.total, icon: 'users', color: 'bg-primary/10 text-primary' },
    { label: 'Inscrits', value: s.inscrits, icon: 'user-plus', color: 'bg-info/10 text-info' },
    { label: 'Confirmes', value: s.confirmes, icon: 'user-check', color: 'bg-success/10 text-success' },
    { label: 'Annules', value: s.annules, icon: 'user-xmark', color: 'bg-error/10 text-error' },
    { label: 'Presents', value: s.presents, icon: 'circle-check', color: 'bg-primary/10 text-primary' },
    { label: 'Absents', value: s.absents, icon: 'circle-xmark', color: 'bg-neutral/10 text-neutral' },
  ]
})

// Format datetime-local pour le binding
const formatDatetimeLocal = (dt: string | null) => {
  if (!dt) return ''
  const d = new Date(dt)
  const pad = (n: number) => n.toString().padStart(2, '0')
  return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}T${pad(d.getHours())}:${pad(d.getMinutes())}`
}

const charger = async () => {
  await chargerDetail(id)
  if (evenementDetail.value) {
    const e = evenementDetail.value
    form.titre = e.titre
    form.description = e.description || ''
    form.type_evenement = e.type_evenement || ''
    form.format = e.format
    form.langue = e.langue || 'fr'
    form.date_heure_debut = formatDatetimeLocal(e.date_heure_debut)
    form.date_heure_fin = formatDatetimeLocal(e.date_heure_fin)
    form.nombre_places = e.nombre_places
    form.pays_id = e.pays_id || ''
    form.ville = e.ville || ''
    form.adresse = e.adresse || ''
    form.lien_en_ligne = e.lien_en_ligne || ''
    form.enregistrement_url = e.enregistrement_url || ''
    form.image_couverture_url = e.image_couverture_url || ''
    form.type_organisateur = e.type_organisateur || 'personnel'
    form.contact_nom = e.contact_nom || ''
    form.contact_email = e.contact_email || ''
    form.contact_telephone = e.contact_telephone || ''
    form.contact_site_web = e.contact_site_web || ''
  }
}

const sauvegarder = async () => {
  erreurLocale.value = null
  if (form.type_organisateur === 'organisation' && !form.contact_nom.trim()) {
    erreurLocale.value = 'Le nom de l\'organisation est requis'
    return
  }
  saving.value = true
  successMsg.value = null
  try {
    const body: any = {
      titre: form.titre.trim(),
      format: form.format,
      langue: form.langue,
      date_heure_debut: form.date_heure_debut,
    }
    if (form.description.trim()) body.description = form.description.trim()
    if (form.type_evenement.trim()) body.type_evenement = form.type_evenement.trim()
    if (form.date_heure_fin) body.date_heure_fin = form.date_heure_fin
    if (form.nombre_places !== null) body.nombre_places = form.nombre_places
    if (form.pays_id.trim()) body.pays_id = form.pays_id.trim()
    if (form.ville.trim()) body.ville = form.ville.trim()
    if (form.adresse.trim()) body.adresse = form.adresse.trim()
    if (form.lien_en_ligne.trim()) body.lien_en_ligne = form.lien_en_ligne.trim()
    body.enregistrement_url = form.enregistrement_url.trim()
    if (form.image_couverture_url.trim()) body.image_couverture_url = form.image_couverture_url.trim()
    body.type_organisateur = form.type_organisateur
    body.contact_nom = form.type_organisateur === 'organisation' ? form.contact_nom.trim() : ''
    body.contact_email = form.contact_email.trim()
    body.contact_telephone = form.contact_telephone.trim()
    body.contact_site_web = form.contact_site_web.trim()

    await modifier(id, body)
    successMsg.value = 'Evenement mis a jour'
    setTimeout(() => { successMsg.value = null }, 3000)
    await chargerDetail(id)
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  } finally { saving.value = false }
}

// Changement etat
const ouvrirEtatModal = () => {
  if (evenementDetail.value) {
    nouvelEtat.value = evenementDetail.value.etat
    showEtatModal.value = true
  }
}

const executerChangerEtat = async () => {
  if (!evenementDetail.value || nouvelEtat.value === evenementDetail.value.etat) return
  etatLoading.value = true
  try {
    await changerEtat(id, nouvelEtat.value)
    showEtatModal.value = false
    await chargerDetail(id)
  } catch {} finally { etatLoading.value = false }
}

// Changement statut inscription
const changementStatutLoading = ref<string | null>(null)

const executerChangerStatutInscription = async (inscriptionId: string, nouveauStatut: string) => {
  changementStatutLoading.value = inscriptionId
  try {
    await changerStatutInscription(id, inscriptionId, nouveauStatut)
    await chargerInscriptions(id)
    await chargerStatsInscriptions(id)
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  } finally { changementStatutLoading.value = null }
}

onMounted(async () => {
  await charger()
  await chargerInscriptions(id)
  await chargerStatsInscriptions(id)
})
</script>

<template>
  <div>
    <AdminPageHeader :titre="evenementDetail?.titre || 'Chargement...'" sous-titre="Modifier l'evenement">
      <template #actions>
        <NuxtLink to="/admin/evenements" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Retour
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div v-if="loading && !evenementDetail" class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <template v-else-if="evenementDetail">
      <!-- Etat actuel -->
      <div class="flex items-center gap-3 mb-4">
        <span :class="['badge', etatBadge(evenementDetail.etat)]">
          {{ etatLabel(evenementDetail.etat) }}
        </span>
        <button class="btn btn-outline btn-xs" @click="ouvrirEtatModal">
          <font-awesome-icon icon="arrows-rotate" class="mr-1" /> Changer etat
        </button>
        <span v-if="evenementDetail.nombre_inscriptions > 0" class="text-sm text-base-content/70">
          {{ evenementDetail.nombre_inscriptions }} inscription(s)
        </span>
      </div>

      <!-- Onglets -->
      <div class="tabs tabs-bordered mb-4">
        <button :class="['tab', { 'tab-active': ongletActif === 'infos' }]" @click="ongletActif = 'infos'">
          <font-awesome-icon icon="info-circle" class="mr-1" /> Informations
        </button>
        <button :class="['tab', { 'tab-active': ongletActif === 'inscriptions' }]" @click="ongletActif = 'inscriptions'">
          <font-awesome-icon icon="users" class="mr-1" /> Inscriptions
          <span v-if="evenementDetail.nombre_inscriptions" class="badge badge-sm ml-1">{{ evenementDetail.nombre_inscriptions }}</span>
        </button>
      </div>

      <!-- Onglet Informations -->
      <div v-if="ongletActif === 'infos'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <div v-if="erreurLocale || error" class="alert alert-error mb-4">
            <font-awesome-icon icon="circle-exclamation" />
            <span>{{ erreurLocale || error }}</span>
          </div>
          <div v-if="successMsg" class="alert alert-success mb-4">
            <font-awesome-icon icon="circle-check" />
            <span>{{ successMsg }}</span>
          </div>

          <form @submit.prevent="sauvegarder" class="space-y-6">
            <!-- Infos de base -->
            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Informations de base</h3>
              <div class="form-control">
                <label class="label"><span class="label-text">Titre *</span></label>
                <input v-model="form.titre" type="text" class="input input-bordered" required>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Description</span></label>
                <textarea v-model="form.description" class="textarea textarea-bordered h-32" />
              </div>
            </div>

            <!-- Type & format -->
            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Type & format</h3>
              <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                <div class="form-control">
                  <label class="label"><span class="label-text">Type d'evenement</span></label>
                  <input v-model="form.type_evenement" type="text" class="input input-bordered" placeholder="Ex: Conference, Atelier">
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Format</span></label>
                  <select v-model="form.format" class="select select-bordered">
                    <option value="presentiel">Presentiel</option>
                    <option value="en_ligne">En ligne</option>
                    <option value="hybride">Hybride</option>
                  </select>
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Langue</span></label>
                  <input v-model="form.langue" type="text" class="input input-bordered" placeholder="Ex: fr, en, ar">
                </div>
              </div>
            </div>

            <!-- Dates -->
            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Dates & capacite</h3>
              <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                <div class="form-control">
                  <label class="label"><span class="label-text">Date et heure de debut *</span></label>
                  <input v-model="form.date_heure_debut" type="datetime-local" class="input input-bordered" required>
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Date et heure de fin</span></label>
                  <input v-model="form.date_heure_fin" type="datetime-local" class="input input-bordered">
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Nombre de places</span></label>
                  <input v-model.number="form.nombre_places" type="number" min="1" class="input input-bordered">
                </div>
              </div>
            </div>

            <!-- Localisation -->
            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Localisation</h3>
              <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                <div class="form-control">
                  <label class="label"><span class="label-text">Territoire</span></label>
                  <input v-model="form.pays_id" type="text" class="input input-bordered" placeholder="UUID du territoire">
                  <label v-if="evenementDetail.pays_nom" class="label"><span class="label-text-alt text-success">{{ evenementDetail.pays_nom }}</span></label>
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Ville</span></label>
                  <input v-model="form.ville" type="text" class="input input-bordered">
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Adresse</span></label>
                  <input v-model="form.adresse" type="text" class="input input-bordered">
                </div>
              </div>
              <div v-if="afficherLienEnLigne" class="form-control">
                <label class="label"><span class="label-text">Lien en ligne</span></label>
                <input v-model="form.lien_en_ligne" type="url" class="input input-bordered" placeholder="https://zoom.us/j/...">
              </div>
              <div class="form-control">
                <label class="label">
                  <span class="label-text">
                    <font-awesome-icon icon="fa-brands fa-youtube" class="text-red-600 mr-1" />
                    Enregistrement vidéo (YouTube)
                  </span>
                </label>
                <input v-model="form.enregistrement_url" type="url" class="input input-bordered" placeholder="https://www.youtube.com/watch?v=...">
                <label class="label"><span class="label-text-alt text-base-content/60">Affiché en lecteur intégré quand l'événement est terminé.</span></label>
              </div>
            </div>

            <!-- Organisateur & contact -->
            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Organisateur & contact</h3>
              <div class="form-control">
                <label class="label"><span class="label-text">Publication *</span></label>
                <div class="flex flex-wrap gap-4">
                  <label class="label cursor-pointer gap-2">
                    <input v-model="form.type_organisateur" type="radio" value="personnel" class="radio radio-primary">
                    <span class="label-text">En nom propre</span>
                  </label>
                  <label class="label cursor-pointer gap-2">
                    <input v-model="form.type_organisateur" type="radio" value="organisation" class="radio radio-primary">
                    <span class="label-text">Au nom d'une organisation</span>
                  </label>
                </div>
              </div>
              <div v-if="form.type_organisateur === 'organisation'" class="form-control">
                <label class="label"><span class="label-text">Nom de l'organisation *</span></label>
                <input v-model="form.contact_nom" type="text" class="input input-bordered" placeholder="Ex: Fondation UAfricas">
              </div>
              <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                <div class="form-control">
                  <label class="label"><span class="label-text">Email de contact</span></label>
                  <input v-model="form.contact_email" type="email" class="input input-bordered" placeholder="contact@exemple.org">
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Téléphone</span></label>
                  <input v-model="form.contact_telephone" type="tel" class="input input-bordered" placeholder="+221 ...">
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Site web</span></label>
                  <input v-model="form.contact_site_web" type="url" class="input input-bordered" placeholder="https://...">
                </div>
              </div>
            </div>

            <!-- Image de couverture -->
            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Image de couverture</h3>
              <div class="form-control">
                <label class="label"><span class="label-text">URL de l'image de couverture</span></label>
                <input v-model="form.image_couverture_url" type="url" class="input input-bordered" placeholder="https://exemple.com/image.jpg">
              </div>
            </div>

            <div class="flex items-center justify-between pt-4">
              <div class="text-sm text-base-content/50">
                <span v-if="evenementDetail.cree_par_nom">Cree par {{ evenementDetail.cree_par_nom }}</span>
                <br>ID: {{ evenementDetail.id.substring(0, 8) }}...
              </div>
              <button type="submit" class="btn btn-primary" :class="{ loading: saving }" :disabled="saving">
                <font-awesome-icon v-if="!saving" icon="floppy-disk" class="mr-1" /> Enregistrer
              </button>
            </div>
          </form>
        </div>
      </div>

      <!-- Onglet Inscriptions -->
      <div v-else-if="ongletActif === 'inscriptions'" class="space-y-4">
        <!-- Stats cards -->
        <div v-if="inscriptionStats" class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-3">
          <AdminStatsCard v-for="(stat, index) in statsCards" :key="index" :stat="stat" />
        </div>

        <!-- Table inscriptions -->
        <div class="card bg-base-100 shadow-sm">
          <div class="card-body">
            <div v-if="!inscriptions.length" class="text-center py-8 text-base-content/50">
              <font-awesome-icon icon="inbox" class="text-4xl mb-2" />
              <p>Aucune inscription pour cet evenement</p>
            </div>
            <div v-else class="overflow-x-auto">
              <table class="table table-zebra">
                <thead>
                  <tr>
                    <th>Participant</th>
                    <th>Email</th>
                    <th class="text-center w-28">Statut</th>
                    <th class="w-28">Date</th>
                    <th class="w-40">Actions</th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="insc in inscriptions" :key="insc.id">
                    <td>{{ insc.prenom }} {{ insc.nom }}</td>
                    <td>{{ insc.email }}</td>
                    <td class="text-center">
                      <span :class="['badge badge-sm', statutBadge(insc.statut)]">
                        {{ statutLabel(insc.statut) }}
                      </span>
                    </td>
                    <td>{{ new Date(insc.created_at).toLocaleDateString('fr-FR') }}</td>
                    <td>
                      <select
                        class="select select-bordered select-xs w-full"
                        :value="insc.statut"
                        :disabled="changementStatutLoading === insc.id"
                        @change="(e: Event) => executerChangerStatutInscription(insc.id, (e.target as HTMLSelectElement).value)"
                      >
                        <option value="inscrit">Inscrit</option>
                        <option value="confirme">Confirme</option>
                        <option value="annule">Annule</option>
                        <option value="present">Present</option>
                        <option value="absent">Absent</option>
                      </select>
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>
      </div>

      <!-- Modal changement etat -->
      <div v-if="showEtatModal" class="modal modal-open">
        <div class="modal-box">
          <h3 class="font-bold text-lg mb-4">Changer l'etat de l'evenement</h3>
          <div class="form-control">
            <select v-model="nouvelEtat" class="select select-bordered">
              <option value="brouillon">Brouillon</option>
              <option value="publie">Publie</option>
              <option value="suspendu">Suspendu</option>
              <option value="annule">Annule</option>
            </select>
          </div>
          <div class="modal-action">
            <button class="btn btn-ghost" @click="showEtatModal = false">Annuler</button>
            <button class="btn btn-primary" :class="{ loading: etatLoading }" :disabled="etatLoading || nouvelEtat === evenementDetail.etat" @click="executerChangerEtat">
              Confirmer
            </button>
          </div>
        </div>
        <div class="modal-backdrop" @click="showEtatModal = false" />
      </div>
    </template>
  </div>
</template>
