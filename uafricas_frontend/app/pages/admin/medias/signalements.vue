<script setup lang="ts">
/**
 * File des contenus signalés (US7).
 *
 * Triée côté serveur par nombre de signalements décroissant : le contenu le
 * plus contesté remonte en tête. Un contenu franchissant le seuil est déjà
 * retiré de l'antenne à son arrivée ici — la décision administrative consiste
 * à le rétablir (compteur remis à zéro) ou à le supprimer définitivement.
 */
import {
  LIBELLES_ETAT_MEDIA,
  LIBELLES_MOTIF_SIGNALEMENT,
  type ContenuSignaleAPI,
  type EtatModerationMedia,
  type SignalementDetailAPI,
} from '~/composables/useAdminMediaSignalements'
import { LIBELLES_TYPE_MEDIA, type TypeMedia } from '~/composables/useMediaSocial'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

useHead({ title: 'Contenus signalés — Administration' })

const { lister, detailSignalements, changerEtat, chargement, erreur } = useAdminMediaSignalements()

const contenus = ref<ContenuSignaleAPI[]>([])
const total = ref(0)
const page = ref(1)
const totalPages = ref(1)

// La file s'ouvre sur ce qui a été retiré de l'antenne : c'est ce qui attend
// une décision. Les contenus signalés mais encore en ligne restent accessibles
// via le filtre.
const filtreSuspendu = ref<'' | 'true' | 'false'>('true')
const filtreType = ref<TypeMedia | ''>('')

const BADGES_ETAT: Record<string, string> = {
  publie: 'badge-success',
  suspendu: 'badge-error',
  en_attente: 'badge-warning',
  supprime: 'badge-ghost',
}

const charger = async () => {
  const res = await lister({
    type_media: filtreType.value || undefined,
    suspendu: filtreSuspendu.value === '' ? undefined : filtreSuspendu.value === 'true',
    page: page.value,
    par_page: 20,
  })
  contenus.value = res?.contenus ?? []
  total.value = res?.total ?? 0
  totalPages.value = res?.total_pages ?? 1
}

watch([filtreSuspendu, filtreType], () => {
  page.value = 1
  charger()
})
watch(page, charger)
onMounted(charger)

// ── Panneau de détail ────────────────────────────────────────────

const contenuExamine = ref<ContenuSignaleAPI | null>(null)
const signalements = ref<SignalementDetailAPI[]>([])
const chargementDetail = ref(false)
const actionEnCours = ref(false)

const examiner = async (contenu: ContenuSignaleAPI) => {
  contenuExamine.value = contenu
  chargementDetail.value = true
  signalements.value = (await detailSignalements(contenu.type_media, contenu.id)) ?? []
  chargementDetail.value = false
}

const fermerDetail = () => {
  contenuExamine.value = null
  signalements.value = []
}

const decider = async (etat: EtatModerationMedia) => {
  const contenu = contenuExamine.value
  if (!contenu || actionEnCours.value) return
  if (etat === 'supprime'
    && !confirm(`Supprimer définitivement « ${contenu.titre} » ? Cette action retire le contenu de l'antenne sans retour possible.`)) {
    return
  }
  actionEnCours.value = true
  const ok = await changerEtat(contenu.type_media, contenu.id, etat)
  actionEnCours.value = false
  if (ok) {
    fermerDetail()
    await charger()
  }
}

const dateFormatee = (iso: string | null) =>
  iso
    ? new Date(iso).toLocaleDateString('fr-FR', {
      day: 'numeric', month: 'short', year: 'numeric', hour: '2-digit', minute: '2-digit',
    })
    : '—'

const auteurNom = (s: SignalementDetailAPI) =>
  `${s.auteur.prenom ?? ''} ${s.auteur.nom ?? ''}`.trim() || 'Membre'
</script>

<template>
  <div class="p-6">
    <header class="mb-6">
      <h1 class="text-2xl font-bold">Contenus signalés</h1>
      <p class="text-sm opacity-70 mt-1">
        Au-delà de 10 signalements distincts, un contenu est retiré de l'antenne
        automatiquement. Le rétablir remet son compteur à zéro.
      </p>
    </header>

    <!-- Filtres -->
    <div class="flex flex-wrap gap-3 mb-6">
      <select v-model="filtreSuspendu" class="select select-bordered select-sm">
        <option value="true">Retirés de l'antenne</option>
        <option value="false">Signalés, encore en ligne</option>
        <option value="">Tous</option>
      </select>
      <select v-model="filtreType" class="select select-bordered select-sm">
        <option value="">Tous les types</option>
        <option v-for="(libelle, type) in LIBELLES_TYPE_MEDIA" :key="type" :value="type">
          {{ libelle }}
        </option>
      </select>
      <span class="text-sm opacity-70 self-center ml-auto">{{ total }} contenu(s)</span>
    </div>

    <div v-if="erreur" class="alert alert-error mb-4">
      <span>{{ erreur }}</span>
    </div>

    <div v-if="chargement" class="flex justify-center py-16">
      <span class="loading loading-spinner loading-lg"></span>
    </div>

    <div v-else-if="contenus.length === 0" class="text-center py-16 opacity-60">
      Aucun contenu ne correspond à ces filtres.
    </div>

    <div v-else class="overflow-x-auto">
      <table class="table table-zebra">
        <thead>
          <tr>
            <th>Contenu</th>
            <th>Type</th>
            <th>Signalements</th>
            <th>Dernier</th>
            <th>État</th>
            <th></th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="contenu in contenus" :key="`${contenu.type_media}-${contenu.id}`">
            <td class="font-medium">
              <NuxtLink
                v-if="contenu.url_detail"
                :to="contenu.url_detail"
                target="_blank"
                class="link link-hover"
              >
                {{ contenu.titre }}
              </NuxtLink>
              <span v-else>{{ contenu.titre }}</span>
            </td>
            <td>{{ LIBELLES_TYPE_MEDIA[contenu.type_media] }}</td>
            <td>
              <span class="badge badge-sm" :class="contenu.nombre_signalements > 10 ? 'badge-error' : 'badge-warning'">
                {{ contenu.nombre_signalements }}
              </span>
            </td>
            <td class="whitespace-nowrap text-sm opacity-70">
              {{ dateFormatee(contenu.dernier_signalement) }}
            </td>
            <td>
              <span class="badge badge-sm" :class="BADGES_ETAT[contenu.etat] ?? 'badge-ghost'">
                {{ LIBELLES_ETAT_MEDIA[contenu.etat] ?? contenu.etat }}
              </span>
            </td>
            <td class="text-right">
              <button class="btn btn-sm btn-primary" @click="examiner(contenu)">
                Examiner
              </button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <div v-if="totalPages > 1" class="join mt-6 flex justify-center">
      <button class="join-item btn btn-sm" :disabled="page <= 1" @click="page--">«</button>
      <span class="join-item btn btn-sm btn-disabled">{{ page }} / {{ totalPages }}</span>
      <button class="join-item btn btn-sm" :disabled="page >= totalPages" @click="page++">»</button>
    </div>

    <!-- Panneau d'arbitrage -->
    <dialog v-if="contenuExamine" class="modal modal-open">
      <div class="modal-box max-w-2xl">
        <h3 class="font-bold text-lg">{{ contenuExamine.titre }}</h3>
        <p class="text-sm opacity-70 mt-1">
          {{ LIBELLES_TYPE_MEDIA[contenuExamine.type_media] }} —
          {{ contenuExamine.nombre_signalements }} signalement(s) —
          {{ LIBELLES_ETAT_MEDIA[contenuExamine.etat] ?? contenuExamine.etat }}
        </p>

        <NuxtLink
          v-if="contenuExamine.url_detail"
          :to="contenuExamine.url_detail"
          target="_blank"
          class="btn btn-xs btn-outline mt-3"
        >
          Voir la page publique
        </NuxtLink>

        <div class="divider">Motifs invoqués</div>

        <div v-if="chargementDetail" class="flex justify-center py-8">
          <span class="loading loading-spinner"></span>
        </div>
        <div v-else-if="signalements.length === 0" class="text-center py-6 opacity-60 text-sm">
          Aucun signalement détaillé.
        </div>
        <ul v-else class="space-y-3 max-h-72 overflow-y-auto">
          <li
            v-for="s in signalements"
            :key="s.id"
            class="border border-base-300 rounded-lg p-3"
          >
            <div class="flex items-start justify-between gap-3">
              <span class="badge badge-sm badge-warning">
                {{ LIBELLES_MOTIF_SIGNALEMENT[s.motif ?? ''] ?? s.motif ?? 'Non précisé' }}
              </span>
              <span class="text-xs opacity-60 whitespace-nowrap">{{ dateFormatee(s.created_at) }}</span>
            </div>
            <p v-if="s.description" class="text-sm mt-2">{{ s.description }}</p>
            <p class="text-xs opacity-60 mt-2">{{ auteurNom(s) }}</p>
          </li>
        </ul>

        <div class="modal-action">
          <button class="btn btn-sm" :disabled="actionEnCours" @click="fermerDetail">
            Fermer
          </button>
          <button
            v-if="contenuExamine.etat !== 'suspendu'"
            class="btn btn-sm btn-warning"
            :disabled="actionEnCours"
            @click="decider('suspendu')"
          >
            Retirer de l'antenne
          </button>
          <button
            v-else
            class="btn btn-sm btn-success"
            :disabled="actionEnCours"
            @click="decider('publie')"
          >
            Rétablir
          </button>
          <button
            class="btn btn-sm btn-error"
            :disabled="actionEnCours"
            @click="decider('supprime')"
          >
            Supprimer
          </button>
        </div>
      </div>
      <form method="dialog" class="modal-backdrop" @click="fermerDetail">
        <button>Fermer</button>
      </form>
    </dialog>
  </div>
</template>
