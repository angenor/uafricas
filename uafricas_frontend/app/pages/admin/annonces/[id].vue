<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const route = useRoute()
const id = route.params.id as string
const {
  annonceDetail, chargerDetail, modifier, changerEtat,
  ajouterPays, retirerPays, ajouterMedia, retirerMedia, reordonnerMedias,
  loading, error,
} = useAdminAnnonces()

const saving = ref(false)
const erreurLocale = ref<string | null>(null)
const successMsg = ref<string | null>(null)
const ongletActif = ref('infos')

// Formulaire principal
const form = reactive({
  titre: '',
  description: '',
  type_operation: 'vente',
  condition_article: 'non_applicable',
  prix: null as number | null,
  devise: 'XOF',
  prix_negociable: false,
  ville: '',
  adresse: '',
  type_contact: 'messagerie_plateforme',
  contact_info: '',
  quantite: 1,
  categorie_id: '',
})

// Etat moderation
const etatAnnonce = ref('')
const etatLoading = ref(false)

// Pays
const nouveauPaysId = ref('')
const paysLoading = ref(false)

// Medias
const nouveauMediaUrl = ref('')
const nouveauMediaType = ref('')
const mediaLoading = ref(false)

const charger = async () => {
  await chargerDetail(id)
  if (annonceDetail.value) {
    const a = annonceDetail.value
    form.titre = a.titre
    form.description = a.description
    form.type_operation = a.type_operation
    form.condition_article = a.condition_article || 'non_applicable'
    form.prix = a.prix
    form.devise = a.devise || 'XOF'
    form.prix_negociable = a.prix_negociable || false
    form.ville = a.ville || ''
    form.adresse = a.adresse || ''
    form.type_contact = a.type_contact || 'messagerie_plateforme'
    form.contact_info = a.contact_info || ''
    form.quantite = a.quantite || 1
    form.categorie_id = a.categorie_id || ''
    etatAnnonce.value = a.etat
  }
}

const sauvegarder = async () => {
  saving.value = true
  erreurLocale.value = null
  successMsg.value = null
  try {
    const body: any = { ...form }
    if (!body.categorie_id) delete body.categorie_id
    if (body.prix === null || body.prix === 0) delete body.prix
    await modifier(id, body)
    successMsg.value = 'Annonce mise a jour'
    setTimeout(() => { successMsg.value = null }, 3000)
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  } finally {
    saving.value = false
  }
}

const executerChangerEtat = async (nouvelEtat: string) => {
  etatLoading.value = true
  try {
    await changerEtat(id, nouvelEtat)
    etatAnnonce.value = nouvelEtat
    successMsg.value = `Etat change en "${nouvelEtat}"`
    setTimeout(() => { successMsg.value = null }, 3000)
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  } finally {
    etatLoading.value = false
  }
}

const executerAjouterPays = async () => {
  if (!nouveauPaysId.value.trim()) return
  paysLoading.value = true
  try {
    await ajouterPays(id, nouveauPaysId.value.trim())
    nouveauPaysId.value = ''
    await chargerDetail(id)
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  } finally {
    paysLoading.value = false
  }
}

const executerRetirerPays = async (paysId: string) => {
  try {
    await retirerPays(id, paysId)
    await chargerDetail(id)
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  }
}

const executerAjouterMedia = async () => {
  if (!nouveauMediaUrl.value.trim()) return
  mediaLoading.value = true
  try {
    await ajouterMedia(id, {
      media_url: nouveauMediaUrl.value.trim(),
      type_mime: nouveauMediaType.value.trim() || undefined,
    })
    nouveauMediaUrl.value = ''
    nouveauMediaType.value = ''
    await chargerDetail(id)
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  } finally {
    mediaLoading.value = false
  }
}

const executerRetirerMedia = async (mediaId: string) => {
  try {
    await retirerMedia(id, mediaId)
    await chargerDetail(id)
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  }
}

const monterMedia = async (index: number, direction: 'up' | 'down') => {
  if (!annonceDetail.value) return
  const medias = [...annonceDetail.value.medias]
  const target = direction === 'up' ? index - 1 : index + 1
  if (target < 0 || target >= medias.length) return

  const ordres = medias.map((m, i) => {
    if (i === index) return { media_id: m.id, ordre: target }
    if (i === target) return { media_id: m.id, ordre: index }
    return { media_id: m.id, ordre: i }
  })

  try {
    await reordonnerMedias(id, ordres)
    await chargerDetail(id)
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  }
}

const etatBadge = (etat: string) => {
  const map: Record<string, string> = {
    publiee: 'badge-success', en_attente: 'badge-warning', brouillon: 'badge-info',
    expiree: 'badge-neutral', suspendue: 'badge-error', supprimee: 'badge-neutral opacity-50',
  }
  return map[etat] || 'badge-neutral'
}

onMounted(() => charger())
</script>

<template>
  <div>
    <AdminPageHeader :titre="annonceDetail?.titre || 'Chargement...'" sous-titre="Modifier l'annonce">
      <template #actions>
        <NuxtLink to="/admin/annonces" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Retour
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div v-if="loading && !annonceDetail" class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <div v-else-if="annonceDetail" class="space-y-4">
      <!-- Alertes -->
      <div v-if="erreurLocale || error" class="alert alert-error">
        <font-awesome-icon icon="circle-exclamation" />
        <span>{{ erreurLocale || error }}</span>
        <button class="btn btn-ghost btn-xs" @click="erreurLocale = null">✕</button>
      </div>
      <div v-if="successMsg" class="alert alert-success">
        <font-awesome-icon icon="circle-check" />
        <span>{{ successMsg }}</span>
      </div>

      <!-- Barre d'etat + moderation -->
      <div class="card bg-base-100 shadow-sm">
        <div class="card-body flex-row items-center justify-between py-3">
          <div class="flex items-center gap-3">
            <span class="text-sm font-medium">Etat :</span>
            <span class="badge" :class="etatBadge(etatAnnonce)">{{ etatAnnonce }}</span>
          </div>
          <div class="flex gap-2">
            <button
              v-if="etatAnnonce !== 'publiee'"
              class="btn btn-success btn-sm"
              :class="{ loading: etatLoading }"
              :disabled="etatLoading"
              @click="executerChangerEtat('publiee')"
            >
              Publier
            </button>
            <button
              v-if="etatAnnonce === 'publiee'"
              class="btn btn-warning btn-sm"
              :class="{ loading: etatLoading }"
              :disabled="etatLoading"
              @click="executerChangerEtat('suspendue')"
            >
              Suspendre
            </button>
            <button
              v-if="etatAnnonce !== 'en_attente' && etatAnnonce !== 'brouillon'"
              class="btn btn-ghost btn-sm"
              :class="{ loading: etatLoading }"
              :disabled="etatLoading"
              @click="executerChangerEtat('en_attente')"
            >
              Mettre en attente
            </button>
          </div>
        </div>
      </div>

      <!-- Onglets -->
      <div role="tablist" class="tabs tabs-bordered">
        <button role="tab" class="tab" :class="{ 'tab-active': ongletActif === 'infos' }" @click="ongletActif = 'infos'">
          <font-awesome-icon icon="circle-info" class="mr-1" /> Informations
        </button>
        <button role="tab" class="tab" :class="{ 'tab-active': ongletActif === 'pays' }" @click="ongletActif = 'pays'">
          <font-awesome-icon icon="earth-africa" class="mr-1" /> Territoires cibles ({{ annonceDetail.pays.length }})
        </button>
        <button role="tab" class="tab" :class="{ 'tab-active': ongletActif === 'medias' }" @click="ongletActif = 'medias'">
          <font-awesome-icon icon="image" class="mr-1" /> Medias ({{ annonceDetail.medias.length }})
        </button>
      </div>

      <!-- Onglet Infos -->
      <div v-show="ongletActif === 'infos'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <form @submit.prevent="sauvegarder" class="space-y-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Titre *</span></label>
              <input v-model="form.titre" type="text" class="input input-bordered" required />
            </div>

            <div class="form-control">
              <label class="label"><span class="label-text">Description *</span></label>
              <textarea v-model="form.description" class="textarea textarea-bordered" rows="4" />
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Type d'operation</span></label>
                <select v-model="form.type_operation" class="select select-bordered">
                  <option value="vente">Vente</option>
                  <option value="troc">Troc</option>
                  <option value="don">Don</option>
                  <option value="association">Association</option>
                  <option value="opportunite">Opportunite</option>
                </select>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Condition</span></label>
                <select v-model="form.condition_article" class="select select-bordered">
                  <option value="neuf">Neuf</option>
                  <option value="occasion">Occasion</option>
                  <option value="reconditionne">Reconditionne</option>
                  <option value="non_applicable">Non applicable</option>
                </select>
              </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Prix</span></label>
                <input v-model.number="form.prix" type="number" class="input input-bordered" min="0" step="0.01" />
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Devise</span></label>
                <select v-model="form.devise" class="select select-bordered">
                  <option value="XOF">XOF</option>
                  <option value="XAF">XAF</option>
                  <option value="EUR">EUR</option>
                  <option value="USD">USD</option>
                  <option value="GBP">GBP</option>
                </select>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Quantite</span></label>
                <input v-model.number="form.quantite" type="number" class="input input-bordered" min="1" />
              </div>
            </div>

            <div class="form-control">
              <label class="label cursor-pointer justify-start gap-3">
                <input v-model="form.prix_negociable" type="checkbox" class="checkbox checkbox-primary" />
                <span class="label-text">Prix negociable</span>
              </label>
            </div>

            <div class="divider">Contact & Localisation</div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Ville</span></label>
                <input v-model="form.ville" type="text" class="input input-bordered" />
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Adresse</span></label>
                <input v-model="form.adresse" type="text" class="input input-bordered" />
              </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Type de contact</span></label>
                <select v-model="form.type_contact" class="select select-bordered">
                  <option value="messagerie_plateforme">Messagerie plateforme</option>
                  <option value="email">Email</option>
                  <option value="telephone">Telephone</option>
                </select>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Info de contact</span></label>
                <input v-model="form.contact_info" type="text" class="input input-bordered" />
              </div>
            </div>

            <div class="form-control">
              <label class="label"><span class="label-text">ID Categorie (UUID)</span></label>
              <input v-model="form.categorie_id" type="text" class="input input-bordered" placeholder="UUID de la categorie" />
              <label v-if="annonceDetail.categorie_nom" class="label">
                <span class="label-text-alt">Actuelle : {{ annonceDetail.categorie_nom }}</span>
              </label>
            </div>

            <div class="flex items-center justify-between pt-4">
              <div class="text-sm text-base-content/50">
                <span v-if="annonceDetail.slug">Slug : {{ annonceDetail.slug }}</span>
                <span class="ml-4">Vues : {{ annonceDetail.nombre_vues }}</span>
                <span class="ml-4">Auteur : {{ annonceDetail.auteur_prenom }} {{ annonceDetail.auteur_nom }}</span>
              </div>
              <button type="submit" class="btn btn-primary" :class="{ loading: saving }" :disabled="saving">
                <font-awesome-icon v-if="!saving" icon="floppy-disk" class="mr-1" /> Enregistrer
              </button>
            </div>
          </form>
        </div>
      </div>

      <!-- Onglet Pays cibles -->
      <div v-show="ongletActif === 'pays'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <h3 class="font-semibold mb-4">Territoires cibles</h3>

          <!-- Ajouter un pays -->
          <div class="flex gap-2 mb-4">
            <input
              v-model="nouveauPaysId"
              type="text"
              class="input input-bordered input-sm flex-1"
              placeholder="UUID du territoire a ajouter"
            />
            <button
              class="btn btn-primary btn-sm"
              :class="{ loading: paysLoading }"
              :disabled="paysLoading || !nouveauPaysId.trim()"
              @click="executerAjouterPays"
            >
              <font-awesome-icon icon="plus" class="mr-1" /> Ajouter
            </button>
          </div>

          <!-- Liste des pays -->
          <div v-if="annonceDetail.pays.length === 0" class="text-center py-8 text-base-content/50">
            <font-awesome-icon icon="earth-africa" class="text-3xl mb-2" />
            <p>Aucun territoire cible</p>
          </div>
          <div v-else class="space-y-2">
            <div
              v-for="p in annonceDetail.pays"
              :key="p.pays_id"
              class="flex items-center justify-between p-3 bg-base-200 rounded-lg"
            >
              <div class="flex items-center gap-2">
                <font-awesome-icon icon="flag" class="text-primary" />
                <span>{{ p.pays_nom }}</span>
              </div>
              <button class="btn btn-ghost btn-xs text-error" @click="executerRetirerPays(p.pays_id)">
                <font-awesome-icon icon="xmark" />
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Onglet Medias -->
      <div v-show="ongletActif === 'medias'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <h3 class="font-semibold mb-4">Galerie medias</h3>

          <!-- Ajouter un media -->
          <div class="flex gap-2 mb-4">
            <input
              v-model="nouveauMediaUrl"
              type="text"
              class="input input-bordered input-sm flex-1"
              placeholder="URL du media (image/video)"
            />
            <input
              v-model="nouveauMediaType"
              type="text"
              class="input input-bordered input-sm w-36"
              placeholder="Type MIME"
            />
            <button
              class="btn btn-primary btn-sm"
              :class="{ loading: mediaLoading }"
              :disabled="mediaLoading || !nouveauMediaUrl.trim()"
              @click="executerAjouterMedia"
            >
              <font-awesome-icon icon="plus" class="mr-1" /> Ajouter
            </button>
          </div>

          <!-- Liste des medias -->
          <div v-if="annonceDetail.medias.length === 0" class="text-center py-8 text-base-content/50">
            <font-awesome-icon icon="image" class="text-3xl mb-2" />
            <p>Aucun media</p>
          </div>
          <div v-else class="space-y-2">
            <div
              v-for="(m, index) in annonceDetail.medias"
              :key="m.id"
              class="flex items-center gap-3 p-3 bg-base-200 rounded-lg"
            >
              <!-- Preview -->
              <div class="avatar">
                <div class="w-16 h-16 rounded">
                  <img
                    v-if="m.type_mime?.startsWith('image') || m.media_url.match(/\.(jpg|jpeg|png|gif|webp)$/i)"
                    :src="m.media_url"
                    alt=""
                    class="object-cover"
                  />
                  <div v-else class="bg-base-300 w-full h-full flex items-center justify-center">
                    <font-awesome-icon icon="video" class="text-xl" />
                  </div>
                </div>
              </div>

              <!-- Infos -->
              <div class="flex-1 min-w-0">
                <p class="text-sm truncate">{{ m.media_url }}</p>
                <p class="text-xs text-base-content/50">
                  {{ m.type_mime || 'Type inconnu' }}
                  <span v-if="m.est_principale" class="badge badge-primary badge-xs ml-1">Principale</span>
                </p>
              </div>

              <!-- Actions reordonnement -->
              <div class="flex flex-col gap-1">
                <button
                  class="btn btn-ghost btn-xs"
                  :disabled="index === 0"
                  @click="monterMedia(index, 'up')"
                >
                  <font-awesome-icon icon="chevron-up" />
                </button>
                <button
                  class="btn btn-ghost btn-xs"
                  :disabled="index === annonceDetail!.medias.length - 1"
                  @click="monterMedia(index, 'down')"
                >
                  <font-awesome-icon icon="chevron-down" />
                </button>
              </div>

              <!-- Supprimer -->
              <button class="btn btn-ghost btn-xs text-error" @click="executerRetirerMedia(m.id)">
                <font-awesome-icon icon="trash" />
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
