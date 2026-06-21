<template>
  <div>
    <!-- Chargement -->
    <div v-if="chargementListe" class="flex justify-center py-10">
      <font-awesome-icon icon="fa-solid fa-spinner" class="text-2xl text-custom-chocolat animate-spin" />
    </div>

    <!-- Vide -->
    <div v-else-if="!mesEvenements.length" class="text-center py-10">
      <font-awesome-icon icon="fa-solid fa-calendar-plus" class="text-4xl text-gray-300 mb-3" />
      <p class="text-gray-500 text-sm mb-4">Vous n'avez pas encore proposé d'événement.</p>
      <NuxtLink
        to="/evenements/liste"
        class="inline-flex items-center gap-2 px-5 py-2.5 text-sm bg-linear-to-r from-custom-chocolat to-custom-green text-white font-medium rounded-xl hover:shadow-lg transition-all"
      >
        <font-awesome-icon icon="fa-solid fa-plus" />
        Proposer un événement
      </NuxtLink>
    </div>

    <!-- Liste -->
    <div v-else class="space-y-4">
      <div
        v-for="ev in mesEvenements"
        :key="ev.id"
        class="border border-gray-200 rounded-xl overflow-hidden"
      >
        <!-- Ligne principale -->
        <div class="flex flex-col sm:flex-row gap-4 p-4">
          <!-- Vignette -->
          <div class="w-full sm:w-32 h-24 shrink-0 rounded-lg overflow-hidden bg-gray-100">
            <img
              v-if="ev.couverture_url"
              :src="ev.couverture_url"
              :alt="ev.titre"
              class="w-full h-full object-cover"
            />
            <div v-else class="w-full h-full flex items-center justify-center text-gray-300">
              <font-awesome-icon icon="fa-regular fa-calendar" class="text-2xl" />
            </div>
          </div>

          <!-- Infos -->
          <div class="flex-1 min-w-0">
            <div class="flex items-start justify-between gap-2">
              <h3 class="font-semibold text-gray-800 truncate">{{ ev.titre }}</h3>
              <span
                class="shrink-0 inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium"
                :class="etatClasses(ev.etat)"
              >
                {{ etatLabel(ev.etat) }}
              </span>
            </div>
            <div class="flex flex-wrap items-center gap-x-4 gap-y-1 mt-1 text-xs text-gray-500">
              <span>
                <font-awesome-icon icon="fa-regular fa-calendar" class="mr-1" />
                {{ formatDateShort(ev.date_heure_debut) }} · {{ getHeure(ev.date_heure_debut) }}
              </span>
              <span>
                <font-awesome-icon icon="fa-solid fa-tag" class="mr-1" />
                {{ ev.type }}
              </span>
              <span v-if="ev.ville || ev.pays">
                <font-awesome-icon icon="fa-solid fa-location-dot" class="mr-1" />
                {{ [ev.ville, ev.pays].filter(Boolean).join(', ') }}
              </span>
              <span>
                <font-awesome-icon icon="fa-solid fa-users" class="mr-1" />
                {{ ev.nombre_inscrits }}<span v-if="ev.nombre_places">/{{ ev.nombre_places }}</span> inscrit(s)
              </span>
            </div>

            <!-- Actions -->
            <div class="flex flex-wrap items-center gap-2 mt-3">
              <NuxtLink
                :to="`/evenements/${ev.id}`"
                class="inline-flex items-center gap-1.5 px-3 py-1.5 text-xs font-medium text-gray-600 border border-gray-200 rounded-lg hover:bg-gray-50 transition-colors"
              >
                <font-awesome-icon icon="fa-solid fa-eye" />
                Voir
              </NuxtLink>

              <button
                class="inline-flex items-center gap-1.5 px-3 py-1.5 text-xs font-medium text-custom-chocolat border border-custom-chocolat/30 rounded-lg hover:bg-orange-50 transition-colors"
                @click="ouvrirEdition(ev)"
              >
                <font-awesome-icon icon="fa-solid fa-pen-to-square" />
                Modifier
              </button>

              <!-- Démarrer le direct (en ligne / hybride, publié) -->
              <NuxtLink
                v-if="peutDiffuser(ev)"
                :to="`/evenements/${ev.id}/direct`"
                class="inline-flex items-center gap-1.5 px-3 py-1.5 text-xs font-medium text-white bg-red-600 rounded-lg hover:brightness-110 transition-all"
              >
                <font-awesome-icon icon="fa-solid fa-tower-broadcast" />
                Démarrer le direct
              </NuxtLink>

              <button
                class="inline-flex items-center gap-1.5 px-3 py-1.5 text-xs font-medium text-gray-600 border border-gray-200 rounded-lg hover:bg-gray-50 transition-colors"
                @click="basculerInscrits(ev.id)"
              >
                <font-awesome-icon icon="fa-solid fa-list-check" />
                Inscrits
              </button>

              <button
                class="inline-flex items-center gap-1.5 px-3 py-1.5 text-xs font-medium text-red-600 border border-red-200 rounded-lg hover:bg-red-50 transition-colors ml-auto"
                @click="confirmerSuppression(ev)"
              >
                <font-awesome-icon icon="fa-solid fa-trash" />
                Supprimer
              </button>
            </div>
          </div>
        </div>

        <!-- Inscrits (dépliable) -->
        <div v-if="inscritsOuverts === ev.id" class="border-t border-gray-100 bg-gray-50 p-4">
          <div v-if="chargementInscrits" class="text-center py-3">
            <font-awesome-icon icon="fa-solid fa-spinner" class="text-custom-chocolat animate-spin" />
          </div>
          <div v-else-if="!inscritsCourants.length" class="text-sm text-gray-400 text-center py-2">
            Aucun inscrit pour le moment.
          </div>
          <ul v-else class="space-y-1.5">
            <li
              v-for="ins in inscritsCourants"
              :key="ins.utilisateur_id"
              class="flex items-center justify-between text-sm bg-white rounded-lg px-3 py-2"
            >
              <span class="font-medium text-gray-700">{{ ins.prenom }} {{ ins.nom }}</span>
              <span class="text-gray-400 text-xs">{{ ins.email }}</span>
            </li>
          </ul>
        </div>
      </div>
    </div>

    <!-- Modal d'édition -->
    <Teleport to="body">
      <div
        v-if="editionOuverte"
        class="z-50 fixed inset-0 bg-black/40 backdrop-blur-xs flex items-center justify-center p-4"
        @click.self="fermerEdition"
      >
        <div class="bg-white w-full max-w-2xl rounded-md border-t-8 border-custom-green pt-5 max-h-[90vh] overflow-y-auto">
          <div class="px-6 pb-6">
            <h2 class="text-xl font-bold text-custom-chocolat mb-4">Modifier l'événement</h2>

            <div v-if="erreurEdition" class="mb-4 bg-red-50 border border-red-200 text-red-600 px-4 py-2 rounded-lg text-sm">
              {{ erreurEdition }}
            </div>

            <div class="space-y-4">
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Titre *</label>
                <input v-model="formEdit.titre" type="text" class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden focus:border-custom-green" />
              </div>

              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Description</label>
                <textarea v-model="formEdit.description" rows="3" class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden focus:border-custom-green resize-none" />
              </div>

              <div class="grid grid-cols-2 gap-4">
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-1">Type *</label>
                  <select v-model="formEdit.type" class="w-full rounded-md border-2 px-2 py-2 border-custom-chocolat text-custom-chocolat focus:outline-hidden">
                    <option value="En ligne">En ligne</option>
                    <option value="En présentiel">En présentiel</option>
                    <option value="Hybride">Hybride</option>
                  </select>
                </div>
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-1">Territoire</label>
                  <select v-model="formEdit.pays" class="w-full rounded-md border-2 px-2 py-2 border-custom-chocolat text-custom-chocolat focus:outline-hidden">
                    <option value="">—</option>
                    <option v-for="p in PAYS_AFRICAINS" :key="p" :value="p">{{ p }}</option>
                  </select>
                </div>
              </div>

              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Ville</label>
                <input v-model="formEdit.ville" type="text" class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden" />
              </div>

              <div v-if="afficheAdresseEdit">
                <label class="block text-sm font-medium text-gray-700 mb-1">Adresse précise</label>
                <input v-model="formEdit.adresse" type="text" class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden" />
              </div>

              <div v-if="afficheLienEdit">
                <label class="block text-sm font-medium text-gray-700 mb-1">Lien de participation en ligne</label>
                <input v-model="formEdit.lien_en_ligne" type="url" class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden" placeholder="https://…" />
              </div>

              <div class="grid grid-cols-2 gap-4">
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-1">Date & heure de début *</label>
                  <input v-model="formEdit.date_heure_debut" type="datetime-local" class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden" />
                </div>
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-1">Date & heure de fin</label>
                  <input v-model="formEdit.date_heure_fin" type="datetime-local" class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden" />
                </div>
              </div>

              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Nombre total de places</label>
                <input v-model.number="formEdit.nombre_places" type="number" min="1" class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden" placeholder="Vide si illimité" />
              </div>

              <!-- Organisateur & contact -->
              <div class="p-3 bg-custom-chocolat/5 border border-custom-chocolat/30 rounded-md space-y-3">
                <p class="text-sm font-semibold text-custom-chocolat">Contact de l'organisateur</p>
                <div class="flex flex-wrap gap-4">
                  <label class="flex items-center gap-2 cursor-pointer">
                    <input v-model="formEdit.type_organisateur" type="radio" value="personnel" class="accent-custom-chocolat" />
                    <span class="text-sm text-gray-700">En mon nom propre</span>
                  </label>
                  <label class="flex items-center gap-2 cursor-pointer">
                    <input v-model="formEdit.type_organisateur" type="radio" value="organisation" class="accent-custom-chocolat" />
                    <span class="text-sm text-gray-700">Au nom d'une organisation</span>
                  </label>
                </div>
                <div v-if="formEdit.type_organisateur === 'organisation'">
                  <label class="block text-sm font-medium text-gray-700 mb-1">Nom de l'organisation *</label>
                  <input v-model="formEdit.contact_nom" type="text" class="w-full border-2 rounded-md p-2 border-custom-chocolat/40 focus:outline-hidden focus:border-custom-chocolat" />
                </div>
                <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                  <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">Email</label>
                    <input v-model="formEdit.contact_email" type="email" class="w-full border-2 rounded-md p-2 border-custom-chocolat/40 focus:outline-hidden focus:border-custom-chocolat" />
                  </div>
                  <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">Téléphone</label>
                    <input v-model="formEdit.contact_telephone" type="tel" class="w-full border-2 rounded-md p-2 border-custom-chocolat/40 focus:outline-hidden focus:border-custom-chocolat" />
                  </div>
                </div>
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-1">Site web</label>
                  <input v-model="formEdit.contact_site_web" type="url" class="w-full border-2 rounded-md p-2 border-custom-chocolat/40 focus:outline-hidden focus:border-custom-chocolat" placeholder="https://…" />
                </div>
              </div>

              <!-- Image de couverture -->
              <div class="p-3 bg-custom-green/20 border border-custom-green rounded-md">
                <label class="block text-sm font-medium text-custom-green mb-2">Image de couverture</label>
                <div class="flex items-center gap-3">
                  <div class="w-24 h-16 shrink-0 rounded-md overflow-hidden bg-gray-100">
                    <img v-if="apercuCouverture" :src="apercuCouverture" alt="Aperçu" class="w-full h-full object-cover" />
                    <div v-else class="w-full h-full flex items-center justify-center text-gray-300">
                      <font-awesome-icon icon="fa-solid fa-image" />
                    </div>
                  </div>
                  <input
                    type="file"
                    accept="image/*"
                    class="flex-1 text-sm text-gray-500 file:mr-4 file:py-2 file:px-4 file:rounded-md file:border-0 file:text-sm file:bg-custom-green file:text-white hover:file:bg-custom-green/90"
                    @change="onFichierCouverture"
                  />
                </div>
                <p class="text-xs text-gray-500 mt-2">Laissez vide pour conserver l'image actuelle.</p>
              </div>
            </div>

            <div class="flex justify-end gap-3 mt-6">
              <button class="px-4 py-2 text-sm text-gray-600 hover:bg-gray-100 rounded-md transition-colors" @click="fermerEdition">
                Annuler
              </button>
              <button
                class="inline-flex items-center gap-2 px-4 py-2 text-sm bg-custom-green text-white rounded-md hover:bg-custom-green/90 transition-colors disabled:opacity-50"
                :disabled="enregistrement"
                @click="enregistrerEdition"
              >
                <font-awesome-icon :icon="enregistrement ? 'fa-solid fa-spinner' : 'fa-solid fa-floppy-disk'" :class="{ 'animate-spin': enregistrement }" />
                Enregistrer
              </button>
            </div>
          </div>
        </div>
      </div>
    </Teleport>

    <!-- Confirmation suppression -->
    <Teleport to="body">
      <div
        v-if="suppressionCible"
        class="z-50 fixed inset-0 bg-black/40 backdrop-blur-xs flex items-center justify-center p-4"
        @click.self="suppressionCible = null"
      >
        <div class="bg-white w-full max-w-md rounded-xl p-6">
          <h3 class="text-lg font-bold text-gray-800 mb-2">Supprimer l'événement ?</h3>
          <p class="text-sm text-gray-500 mb-5">
            « {{ suppressionCible.titre }} » sera retiré définitivement. Cette action est irréversible.
          </p>
          <div class="flex justify-end gap-3">
            <button class="px-4 py-2 text-sm text-gray-600 hover:bg-gray-100 rounded-md" @click="suppressionCible = null">
              Annuler
            </button>
            <button
              class="inline-flex items-center gap-2 px-4 py-2 text-sm bg-red-600 text-white rounded-md hover:bg-red-700 disabled:opacity-50"
              :disabled="suppressionEnCours"
              @click="executerSuppression"
            >
              <font-awesome-icon :icon="suppressionEnCours ? 'fa-solid fa-spinner' : 'fa-solid fa-trash'" :class="{ 'animate-spin': suppressionEnCours }" />
              Supprimer
            </button>
          </div>
        </div>
      </div>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import {
  useEvenements, formatDateShort, getHeure, PAYS_AFRICAINS,
  type EvenementDetailAPI, type InscritEvenement, type ModifierMonEvenementPayload,
} from '~/composables/useEvenements'

const {
  listerMesEvenements, modifierMonEvenement, supprimerMonEvenement,
  changerCouvertureMonEvenement, listerInscritsMonEvenement,
} = useEvenements()

const mesEvenements = ref<EvenementDetailAPI[]>([])
const chargementListe = ref(true)

const charger = async () => {
  chargementListe.value = true
  mesEvenements.value = await listerMesEvenements()
  chargementListe.value = false
}
onMounted(charger)

// ── Badges d'état ──
const etatLabel = (etat: string) => {
  const map: Record<string, string> = {
    brouillon: 'Brouillon',
    publie: 'Publié',
    annule: 'Annulé',
    termine: 'Terminé',
    suspendu: 'Suspendu',
  }
  return map[etat] || etat
}
const etatClasses = (etat: string) => {
  switch (etat) {
    case 'publie': return 'bg-green-100 text-green-700'
    case 'brouillon': return 'bg-amber-100 text-amber-700'
    case 'annule': return 'bg-red-100 text-red-700'
    case 'suspendu': return 'bg-red-100 text-red-700'
    default: return 'bg-gray-100 text-gray-600'
  }
}

// Diffusion possible : événement en ligne/hybride et publié.
const peutDiffuser = (ev: EvenementDetailAPI) => {
  const t = ev.type || ''
  return ev.etat === 'publie' && (t.includes('ligne') || t.includes('Hybride') || t.includes('hybride'))
}

// ── Inscrits (dépliable) ──
const inscritsOuverts = ref<string | null>(null)
const inscritsCourants = ref<InscritEvenement[]>([])
const chargementInscrits = ref(false)

const basculerInscrits = async (id: string) => {
  if (inscritsOuverts.value === id) {
    inscritsOuverts.value = null
    return
  }
  inscritsOuverts.value = id
  chargementInscrits.value = true
  inscritsCourants.value = await listerInscritsMonEvenement(id)
  chargementInscrits.value = false
}

// ── Édition ──
const editionOuverte = ref(false)
const editionId = ref<string | null>(null)
const enregistrement = ref(false)
const erreurEdition = ref<string | null>(null)

const formEdit = reactive({
  titre: '',
  description: '',
  type: 'En présentiel',
  pays: '',
  ville: '',
  adresse: '',
  date_heure_debut: '',
  date_heure_fin: '',
  lien_en_ligne: '',
  nombre_places: null as number | null,
  type_organisateur: 'personnel' as 'personnel' | 'organisation',
  contact_nom: '',
  contact_email: '',
  contact_telephone: '',
  contact_site_web: '',
})

const afficheAdresseEdit = computed(() => formEdit.type === 'En présentiel' || formEdit.type === 'Hybride')
const afficheLienEdit = computed(() => formEdit.type === 'En ligne' || formEdit.type === 'Hybride')

// Couverture : fichier choisi + aperçu (nouveau fichier sinon image actuelle).
const fichierCouverture = ref<File | null>(null)
const apercuLocal = ref<string | null>(null)
const couvertureActuelle = ref<string | null>(null)
const apercuCouverture = computed(() => apercuLocal.value || couvertureActuelle.value)

const onFichierCouverture = (e: Event) => {
  const input = e.target as HTMLInputElement
  const f = input.files?.[0]
  if (!f) return
  fichierCouverture.value = f
  if (apercuLocal.value) URL.revokeObjectURL(apercuLocal.value)
  apercuLocal.value = URL.createObjectURL(f)
}

// Convertit une date ISO en valeur pour <input datetime-local>.
const toLocalInput = (iso: string | null) => {
  if (!iso) return ''
  const d = new Date(iso)
  const pad = (n: number) => n.toString().padStart(2, '0')
  return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}T${pad(d.getHours())}:${pad(d.getMinutes())}`
}

const ouvrirEdition = (ev: EvenementDetailAPI) => {
  erreurEdition.value = null
  editionId.value = ev.id
  fichierCouverture.value = null
  if (apercuLocal.value) { URL.revokeObjectURL(apercuLocal.value); apercuLocal.value = null }
  couvertureActuelle.value = ev.couverture_url
  formEdit.titre = ev.titre
  formEdit.description = ev.description || ''
  formEdit.type = ev.type || 'En présentiel'
  formEdit.pays = ev.pays || ''
  formEdit.ville = ev.ville || ''
  formEdit.adresse = ev.adresse || ''
  formEdit.date_heure_debut = toLocalInput(ev.date_heure_debut)
  formEdit.date_heure_fin = toLocalInput(ev.date_heure_fin)
  formEdit.lien_en_ligne = ev.lien_en_ligne || ''
  formEdit.nombre_places = ev.nombre_places
  formEdit.type_organisateur = ev.type_organisateur || 'personnel'
  formEdit.contact_nom = ev.contact_nom || ''
  formEdit.contact_email = ev.contact_email || ''
  formEdit.contact_telephone = ev.contact_telephone || ''
  formEdit.contact_site_web = ev.contact_site_web || ''
  editionOuverte.value = true
}

const fermerEdition = () => {
  editionOuverte.value = false
  editionId.value = null
}

const enregistrerEdition = async () => {
  if (!editionId.value) return
  if (!formEdit.titre.trim()) { erreurEdition.value = 'Le titre est requis'; return }
  if (formEdit.type_organisateur === 'organisation' && !formEdit.contact_nom.trim()) {
    erreurEdition.value = "Le nom de l'organisation est requis"
    return
  }
  enregistrement.value = true
  erreurEdition.value = null

  const payload: ModifierMonEvenementPayload = {
    titre: formEdit.titre.trim(),
    description: formEdit.description,
    type: formEdit.type,
    pays: formEdit.pays,
    ville: formEdit.ville,
    adresse: afficheAdresseEdit.value ? formEdit.adresse : '',
    date_heure_debut: formEdit.date_heure_debut,
    date_heure_fin: formEdit.date_heure_fin,
    lien_en_ligne: afficheLienEdit.value ? formEdit.lien_en_ligne : '',
    nombre_places: formEdit.nombre_places,
    type_organisateur: formEdit.type_organisateur,
    contact_nom: formEdit.type_organisateur === 'organisation' ? formEdit.contact_nom : '',
    contact_email: formEdit.contact_email,
    contact_telephone: formEdit.contact_telephone,
    contact_site_web: formEdit.contact_site_web,
  }

  const maj = await modifierMonEvenement(editionId.value, payload)
  if (!maj) {
    enregistrement.value = false
    erreurEdition.value = 'La modification a échoué. Vérifiez les champs.'
    return
  }

  // Remplacement éventuel de l'image de couverture.
  if (fichierCouverture.value) {
    const url = await changerCouvertureMonEvenement(editionId.value, fichierCouverture.value)
    if (url) maj.couverture_url = url
  }

  enregistrement.value = false
  const i = mesEvenements.value.findIndex(e => e.id === maj.id)
  if (i !== -1) mesEvenements.value[i] = maj
  fermerEdition()
}

// ── Suppression ──
const suppressionCible = ref<EvenementDetailAPI | null>(null)
const suppressionEnCours = ref(false)

const confirmerSuppression = (ev: EvenementDetailAPI) => { suppressionCible.value = ev }

const executerSuppression = async () => {
  if (!suppressionCible.value) return
  suppressionEnCours.value = true
  const ok = await supprimerMonEvenement(suppressionCible.value.id)
  suppressionEnCours.value = false
  if (ok) {
    mesEvenements.value = mesEvenements.value.filter(e => e.id !== suppressionCible.value!.id)
    suppressionCible.value = null
  }
}
</script>
