<script setup lang="ts">
/**
 * Édition d'une **chaîne TV**.
 *
 * Les programmes ne s'éditent plus ici : depuis 09q ce sont des `emission_*`,
 * gérées sur `/admin/medias/emissions`. Le tableau du bas ne fait plus que les
 * lister, et la vedette générale a suivi les épisodes : c'est un épisode, pas
 * un programme, qui occupe la tête de `/medias/tele`.
 *
 * Thématiques et couverture : exigées seulement à l'état `publie` (FR-029,
 * FR-035). Sur un brouillon elles restent facultatives, sans quoi on ne
 * pourrait plus enregistrer une fiche en cours de saisie.
 */
import type { AdminEmission } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const route = useRoute()
const id = route.params.id as string

const {
  chaineDetail, chargerChaine, modifierChaine,
  ORIGINES_PUBLICATION_TELE, loading, error,
} = useAdminTelevision()
const { listerPays } = useCentresCulturels()
const {
  listerReferentielsEdition, obtenirThematiques, definirThematiques,
  obtenirCouverture, definirCouverture,
} = useMediaSupport()
const { chargerEmissions, filtres: filtresEmissions, emissions, ETATS_EMISSION, libelleCadence } = useAdminMediaEmissions()

const paysDisponibles = ref<{ id: string; nom: string }[]>([])
const thematiquesRef = ref<{ id: string; nom: string }[]>([])
const territoiresRef = ref<{ id: string; nom: string }[]>([])

const thematiquesChoisies = ref<string[]>([])
const couvertureContinentale = ref(false)
const territoiresChoisis = ref<string[]>([])

const saving = ref(false)
const erreurLocale = ref<string | null>(null)
const successMsg = ref<string | null>(null)

const chaineForm = reactive({
  nom: '', description: '', stream_url: '', image_couverture_url: '',
  categorie: 'generaliste', pays_id: '', langue: '', est_en_direct: false,
  origine_publication: 'territoire',
  contact_email: '', contact_telephone: '', contact_whatsapp: '',
  contact_site_web: '', contact_adresse: '',
})

const aideOrigine = computed(() => ORIGINES_PUBLICATION_TELE.find(o => o.valeur === chaineForm.origine_publication)?.aide || '')

const titreDetail = computed(() => chaineDetail.value?.nom || 'Chargement...')
const etatCourant = computed(() => chaineDetail.value?.etat || '')
const detailCharge = computed(() => !!chaineDetail.value)
/** Une chaîne publiée doit porter thématique ET couverture. */
const exigeFiche = computed(() => etatCourant.value === 'publie')

const etatBadge = (etat: string) => ({ brouillon: 'badge-warning', publie: 'badge-success', suspendu: 'badge-error', supprime: 'badge-ghost' } as Record<string, string>)[etat] || 'badge-info'
const etatLabel = (etat: string) => ({ brouillon: 'Brouillon', publie: 'Publie', suspendu: 'Suspendu', supprime: 'Supprime' } as Record<string, string>)[etat] || etat

const showEtatModal = ref(false)
const nouvelEtat = ref('')
const etatLoading = ref(false)
const ouvrirEtatModal = () => { nouvelEtat.value = etatCourant.value; showEtatModal.value = true }

/**
 * Publier exige la fiche complète. Le refus est prononcé ici avant l'appel :
 * le serveur refuserait de toute façon l'écriture des thématiques vides, mais
 * la chaîne serait déjà passée « publiée » entre-temps.
 */
const executerChangerEtat = async () => {
  if (nouvelEtat.value === etatCourant.value) return
  if (nouvelEtat.value === 'publie' && !ficheComplete.value) {
    erreurLocale.value = 'Publier une chaîne suppose au moins une thématique et une couverture territoriale.'
    showEtatModal.value = false
    return
  }
  etatLoading.value = true
  try {
    await modifierChaine(id, { etat: nouvelEtat.value } as any)
    showEtatModal.value = false
    successMsg.value = `Etat changé en "${etatLabel(nouvelEtat.value)}"`
    setTimeout(() => { successMsg.value = null }, 3000)
    await charger()
  }
  catch (e: any) { erreurLocale.value = e?.data?.error || e?.message || 'Erreur' }
  finally { etatLoading.value = false }
}

const ficheComplete = computed(() =>
  thematiquesChoisies.value.length > 0
  && (couvertureContinentale.value || territoiresChoisis.value.length > 0),
)

const charger = async () => {
  await chargerChaine(id)
  if (chaineDetail.value) {
    const c = chaineDetail.value
    chaineForm.nom = c.nom
    chaineForm.description = c.description || ''
    chaineForm.stream_url = c.stream_url || ''
    chaineForm.image_couverture_url = c.image_couverture_url || ''
    chaineForm.categorie = c.categorie || 'generaliste'
    chaineForm.pays_id = c.pays_id || ''
    chaineForm.langue = c.langue || ''
    chaineForm.est_en_direct = c.est_en_direct || false
    chaineForm.origine_publication = c.origine_publication || 'territoire'
    chaineForm.contact_email = c.contact_email || ''
    chaineForm.contact_telephone = c.contact_telephone || ''
    chaineForm.contact_whatsapp = c.contact_whatsapp || ''
    chaineForm.contact_site_web = c.contact_site_web || ''
    chaineForm.contact_adresse = c.contact_adresse || ''
  }
}

const chargerFiche = async () => {
  const [themes, couverture] = await Promise.all([
    obtenirThematiques('chaine_tv', id, true),
    obtenirCouverture('chaine_tv', id, true),
  ])
  thematiquesChoisies.value = themes.map(t => t.id)
  couvertureContinentale.value = couverture?.couverture_continentale ?? false
  territoiresChoisis.value = couverture?.territoires.map(t => t.id) ?? []
}

const sauvegarder = async () => {
  saving.value = true
  erreurLocale.value = null
  successMsg.value = null
  try {
    if (exigeFiche.value && !ficheComplete.value) {
      erreurLocale.value = 'Une chaîne publiée doit porter au moins une thématique et une couverture territoriale.'
      return
    }
    const body: any = {
      nom: chaineForm.nom.trim(),
      stream_url: chaineForm.stream_url.trim(),
      categorie: chaineForm.categorie,
      est_en_direct: chaineForm.est_en_direct,
      origine_publication: chaineForm.origine_publication,
      // Envoyés même vides : c'est ce qui permet d'effacer un contact.
      contact_email: chaineForm.contact_email.trim(),
      contact_telephone: chaineForm.contact_telephone.trim(),
      contact_whatsapp: chaineForm.contact_whatsapp.trim(),
      contact_site_web: chaineForm.contact_site_web.trim(),
      contact_adresse: chaineForm.contact_adresse.trim(),
    }
    if (chaineForm.description.trim()) body.description = chaineForm.description.trim()
    if (chaineForm.image_couverture_url.trim()) body.image_couverture_url = chaineForm.image_couverture_url.trim()
    if (chaineForm.pays_id) body.pays_id = chaineForm.pays_id
    if (chaineForm.langue.trim()) body.langue = chaineForm.langue.trim()
    await modifierChaine(id, body)

    // Thématiques et couverture ont leurs propres endpoints (remplacement
    // intégral) : les greffer au PUT de la chaîne aurait mêlé deux invariants
    // que le serveur valide séparément.
    await definirThematiques('chaine_tv', id, thematiquesChoisies.value, true)
    await definirCouverture('chaine_tv', id, couvertureContinentale.value, territoiresChoisis.value, true)

    successMsg.value = 'Mis à jour avec succès'
    setTimeout(() => { successMsg.value = null }, 3000)
    await charger()
  }
  catch (e: any) { erreurLocale.value = e?.data?.error || e?.message || 'Erreur' }
  finally { saving.value = false }
}

const programmesChaine = computed<AdminEmission[]>(() => emissions.value)

onMounted(async () => {
  await charger()
  const [pays, referentiels] = await Promise.all([listerPays(), listerReferentielsEdition()])
  paysDisponibles.value = pays
  thematiquesRef.value = referentiels.thematiques
  territoiresRef.value = referentiels.territoires
  await chargerFiche()
  filtresEmissions.type = 'tele'
  filtresEmissions.support_id = id
  await chargerEmissions()
})
</script>

<template>
  <div>
    <AdminPageHeader :titre="titreDetail" sous-titre="Modifier la chaîne TV">
      <template #actions>
        <NuxtLink to="/admin/television" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Retour
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div v-if="loading && !detailCharge" class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <template v-else-if="detailCharge">
      <div class="flex items-center gap-3 mb-4">
        <span :class="['badge', etatBadge(etatCourant)]">{{ etatLabel(etatCourant) }}</span>
        <button class="btn btn-outline btn-xs" @click="ouvrirEtatModal">
          <font-awesome-icon icon="arrows-rotate" class="mr-1" /> Changer état
        </button>
      </div>

      <div v-if="erreurLocale || error" class="alert alert-error mb-4">
        <font-awesome-icon icon="circle-exclamation" />
        <span>{{ erreurLocale || error }}</span>
        <button class="btn btn-ghost btn-xs" @click="erreurLocale = null">x</button>
      </div>
      <div v-if="successMsg" class="alert alert-success mb-4">
        <font-awesome-icon icon="circle-check" />
        <span>{{ successMsg }}</span>
      </div>

      <div class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <form class="space-y-6" @submit.prevent="sauvegarder">
            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Informations de base</h3>
              <div class="form-control">
                <label class="label"><span class="label-text">Nom de la chaîne *</span></label>
                <input v-model="chaineForm.nom" type="text" class="input input-bordered" required>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">URL de flux live (optionnel)</span></label>
                <input v-model="chaineForm.stream_url" type="text" class="input input-bordered" placeholder="https://stream.example.com/live">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Description</span></label>
                <textarea v-model="chaineForm.description" class="textarea textarea-bordered h-32" />
              </div>
              <OpportuniteAfriqueImageUploadField v-model="chaineForm.image_couverture_url" label="Image de couverture (optionnel)" />
            </div>

            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Classification</h3>
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div class="form-control">
                  <label class="label"><span class="label-text">Catégorie</span></label>
                  <select v-model="chaineForm.categorie" class="select select-bordered">
                    <option value="generaliste">Généraliste</option>
                    <option value="info">Info</option>
                    <option value="sport">Sport</option>
                    <option value="culture">Culture</option>
                    <option value="divertissement">Divertissement</option>
                    <option value="education">Éducation</option>
                    <option value="musique">Musique</option>
                  </select>
                </div>
                <div class="form-control">
                  <label class="label"><span class="label-text">Langue</span></label>
                  <input v-model="chaineForm.langue" type="text" class="input input-bordered">
                </div>
              </div>
            </div>

            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Localisation &amp; diffusion</h3>
              <div class="form-control">
                <label class="label"><span class="label-text">Origine de publication *</span></label>
                <select v-model="chaineForm.origine_publication" class="select select-bordered">
                  <option v-for="o in ORIGINES_PUBLICATION_TELE" :key="o.valeur" :value="o.valeur">{{ o.libelle }}</option>
                </select>
                <label class="label"><span class="label-text-alt">{{ aideOrigine }}</span></label>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Territoire (siège)</span></label>
                <select v-model="chaineForm.pays_id" class="select select-bordered">
                  <option value="">Aucun</option>
                  <option v-for="p in paysDisponibles" :key="p.id" :value="p.id">{{ p.nom }}</option>
                </select>
              </div>
              <div class="form-control">
                <label class="label cursor-pointer justify-start gap-3">
                  <input v-model="chaineForm.est_en_direct" type="checkbox" class="checkbox checkbox-primary">
                  <span class="label-text">En direct actuellement</span>
                </label>
              </div>
            </div>

            <div class="space-y-4">
              <h3 class="text-lg font-semibold border-b pb-2">Thématiques &amp; couverture</h3>
              <MediaSelecteurThematiques
                v-model="thematiquesChoisies"
                :options="thematiquesRef"
                :requis="exigeFiche"
              />
              <MediaSelecteurCouverture
                :continentale="couvertureContinentale"
                :territoires="territoiresChoisis"
                :options="territoiresRef"
                :requis="exigeFiche"
                @update:continentale="couvertureContinentale = $event"
                @update:territoires="territoiresChoisis = $event"
              />
              <p v-if="!exigeFiche" class="text-sm text-base-content/60">
                Facultatives sur un brouillon ; exigées pour publier la chaîne.
              </p>
            </div>

            <AdminContactsSupportFields
              v-model:email="chaineForm.contact_email"
              v-model:telephone="chaineForm.contact_telephone"
              v-model:whatsapp="chaineForm.contact_whatsapp"
              v-model:site-web="chaineForm.contact_site_web"
              v-model:adresse="chaineForm.contact_adresse"
              libelle-support="la chaîne"
            />

            <div class="flex items-center justify-between pt-4">
              <div class="text-sm text-base-content/50">
                <span v-if="chaineDetail?.cree_par_nom">Créé par {{ chaineDetail.cree_par_nom }}</span>
                <br>ID: {{ chaineDetail?.id?.substring(0, 8) }}...
              </div>
              <button type="submit" class="btn btn-primary" :class="{ loading: saving }" :disabled="saving">
                <font-awesome-icon v-if="!saving" icon="floppy-disk" class="mr-1" /> Enregistrer
              </button>
            </div>
          </form>
        </div>
      </div>

      <!-- Équipe éditoriale (010). Le composant est le MÊME que côté membre 
           les règles ne diffèrent pas, seule l'autorité change : d'où
           `base="admin"`. Il est en Tailwind pur, ce qui est assumé sur une page
           daisyUI : c'est le prix d'un composant unique, et le prix inverse
           serait deux formulaires à tenir d'accord. -->
      <div class="card bg-base-100 shadow-sm mt-6">
        <div class="card-body">
          <MediaGestionEquipe
            type-porteur="chaine_tv"
            :porteur-id="id"
            base="admin"
            titre="Équipe éditoriale de la chaîne"
          />
        </div>
      </div>

      <!-- Programmes de cette chaîne : listés ici, édités sur leur propre écran -->
      <div class="card bg-base-100 shadow-sm mt-6">
        <div class="card-body">
          <div class="flex items-center justify-between gap-3">
            <h3 class="text-lg font-semibold">Programmes de cette chaîne</h3>
            <NuxtLink :to="`/admin/medias/emissions?type=tele&support_id=${id}`" class="btn btn-primary btn-sm">
              <font-awesome-icon icon="film" class="mr-1" /> Gérer les programmes
            </NuxtLink>
          </div>
          <p class="text-sm text-base-content/60">
            Un programme regroupe ses <strong>épisodes</strong>. La grille de programmation vise le programme ;
            l'épisode diffusé se déduit de la rotation.
          </p>

          <div v-if="programmesChaine.length" class="overflow-x-auto mt-2">
            <table class="table table-zebra">
              <thead>
                <tr>
                  <th>Programme</th>
                  <th class="w-28 text-center">Cadence</th>
                  <th class="w-24 text-center">Épisodes</th>
                  <th class="w-28 text-center">État</th>
                  <th class="w-24" />
                </tr>
              </thead>
              <tbody>
                <tr v-for="p in programmesChaine" :key="p.id">
                  <td class="font-medium">{{ p.titre }}</td>
                  <td class="text-center text-sm">{{ libelleCadence(p.cadence) }}</td>
                  <td class="text-center">
                    {{ p.nombre_episodes }}
                    <span v-if="p.episodes_en_attente" class="badge badge-warning badge-sm ml-1">
                      {{ p.episodes_en_attente }} en attente
                    </span>
                  </td>
                  <td class="text-center">
                    <span :class="['badge badge-sm', ETATS_EMISSION[p.etat]?.badge || 'badge-info']">
                      {{ ETATS_EMISSION[p.etat]?.libelle || p.etat }}
                    </span>
                  </td>
                  <td>
                    <NuxtLink :to="`/admin/medias/emissions/${p.id}`" class="btn btn-ghost btn-xs">
                      <font-awesome-icon icon="pen" class="mr-1" /> Ouvrir
                    </NuxtLink>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
          <div v-else class="text-center py-8 text-base-content/50">
            <font-awesome-icon icon="video" class="text-3xl mb-2" />
            <p>Aucun programme rattaché à cette chaîne pour l'instant.</p>
          </div>
        </div>
      </div>

      <!-- Modal changement d'état -->
      <div v-if="showEtatModal" class="modal modal-open">
        <div class="modal-box">
          <h3 class="font-bold text-lg mb-4">Changer l'état</h3>
          <div v-if="!ficheComplete" class="alert alert-warning mb-3">
            <font-awesome-icon icon="triangle-exclamation" />
            <span>Thématique ou couverture manquante : la publication sera refusée.</span>
          </div>
          <div class="form-control">
            <select v-model="nouvelEtat" class="select select-bordered">
              <option value="brouillon">Brouillon</option>
              <option value="publie">Publie</option>
              <option value="suspendu">Suspendu</option>
            </select>
          </div>
          <div class="modal-action">
            <button class="btn btn-ghost" @click="showEtatModal = false">Annuler</button>
            <button class="btn btn-primary" :class="{ loading: etatLoading }" :disabled="etatLoading || nouvelEtat === etatCourant" @click="executerChangerEtat">
              Confirmer
            </button>
          </div>
        </div>
        <div class="modal-backdrop" @click="showEtatModal = false" />
      </div>
    </template>
  </div>
</template>
