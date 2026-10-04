<script setup lang="ts">
/**
 * Revue des épreuves candidates (feature 013).
 *
 * Deux gestes sur cet écran :
 *  1. faire PROPOSER des épreuves à partir du contenu déjà publié d'un module ;
 *  2. PASSER EN REVUE ce qui a été proposé : accepter, corriger, rejeter.
 *
 * Une candidate n'est jamais servie à un membre : seule l'acceptation la rend
 * jouable. Une forme rejetée sur un contenu n'est jamais reproposée.
 */
import {
  messageErreurAdminJeu,
  type BilanDerivationAPI,
  type BilanRevueAPI,
  LIBELLES_TYPE_REPONSE,
  type EpreuveAdminAPI,
  type FormeDerivationAPI,
  type ModuleAdminAPI,
} from '~/composables/useAdminJeu'
import type { ApiResponse, PaginatedResponse } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { listerModules, listerFormes, deriver, revue } = useAdminJeu()
const { adminFetch } = useAdmin()

const modules = ref<ModuleAdminAPI[]>([])
const formes = ref<FormeDerivationAPI[]>([])
const epreuves = ref<EpreuveAdminAPI[]>([])
const total = ref(0)
const page = ref(1)
const PAR_PAGE = 50

const filtreModule = ref('')
const filtreEtat = ref<'candidate' | 'a_revoir'>('candidate')

const selection = ref<Set<string>>(new Set())
const chargement = ref(true)
const enCours = ref(false)
const erreur = ref('')
const message = ref('')
const bilanDerivation = ref<BilanDerivationAPI | null>(null)
const bilanRevue = ref<BilanRevueAPI | null>(null)

const rejetOuvert = ref(false)
const motifRejet = ref('')

const totalPages = computed(() => Math.max(1, Math.ceil(total.value / PAR_PAGE)))
const toutSelectionne = computed(
  () => epreuves.value.length > 0 && epreuves.value.every(e => selection.value.has(e.id)),
)

/** Formes regroupées par module ; un module sans forme reste listé, vide. */
const formesParModule = computed(() =>
  modules.value.map(m => ({
    module: m,
    formes: formes.value.filter(f => f.module === m.code),
  })),
)

const restantes = (f: FormeDerivationAPI) => f.sources_eligibles - f.deja_proposees

const chargerEpreuves = async () => {
  const reponse = await adminFetch<ApiResponse<PaginatedResponse<EpreuveAdminAPI>>>(
    '/api/admin/jeu/epreuves',
    { params: {
      etat: filtreEtat.value,
      module: filtreModule.value,
      page: page.value,
      par_page: PAR_PAGE,
      tri_par: 'module_code',
      tri_dir: 'asc',
    } },
  )
  epreuves.value = reponse.data?.data ?? []
  total.value = reponse.data?.total ?? 0
  // La sélection ne survit pas à un changement de liste : on n'agit que sur ce
  // qu'on voit.
  selection.value = new Set()
}

const rafraichir = async () => {
  erreur.value = ''
  try {
    // La liste d'abord : sa lecture bascule en « à revoir » les épreuves dont
    // la source a changé, ce que les décomptes par module reflètent ensuite.
    await chargerEpreuves()
    const [m, f] = await Promise.all([listerModules(), listerFormes()])
    modules.value = m
    formes.value = f
  }
  catch (e) {
    erreur.value = messageErreurAdminJeu(e, 'Impossible de charger la revue.')
  }
  finally {
    chargement.value = false
  }
}

const notifier = (texte: string) => {
  message.value = texte
  setTimeout(() => { if (message.value === texte) message.value = '' }, 5000)
}

const proposer = async (module: string) => {
  erreur.value = ''
  bilanDerivation.value = null
  enCours.value = true
  try {
    bilanDerivation.value = (await deriver(module)) ?? null
    filtreModule.value = module
    filtreEtat.value = 'candidate'
    page.value = 1
    await rafraichir()
  }
  catch (e) {
    erreur.value = messageErreurAdminJeu(e)
  }
  finally {
    enCours.value = false
  }
}

const basculer = (id: string) => {
  const suivant = new Set(selection.value)
  if (suivant.has(id)) suivant.delete(id)
  else suivant.add(id)
  selection.value = suivant
}

const basculerTout = () => {
  selection.value = toutSelectionne.value ? new Set() : new Set(epreuves.value.map(e => e.id))
}

const decider = async (decision: 'accepter' | 'rejeter', ids: string[], motif?: string) => {
  if (ids.length === 0) return
  erreur.value = ''
  bilanRevue.value = null
  enCours.value = true
  try {
    const bilan = await revue(ids, decision, motif)
    bilanRevue.value = bilan ?? null
    if (bilan) {
      notifier(decision === 'accepter'
        ? `${bilan.acceptees} épreuve${bilan.acceptees > 1 ? 's' : ''} acceptée${bilan.acceptees > 1 ? 's' : ''}.`
        : `${bilan.rejetees} épreuve${bilan.rejetees > 1 ? 's' : ''} rejetée${bilan.rejetees > 1 ? 's' : ''}.`)
    }
    await rafraichir()
  }
  catch (e) {
    erreur.value = messageErreurAdminJeu(e)
  }
  finally {
    enCours.value = false
  }
}

const confirmerRejet = async () => {
  if (!motifRejet.value.trim()) return
  const ids = [...selection.value]
  rejetOuvert.value = false
  await decider('rejeter', ids, motifRejet.value.trim())
  motifRejet.value = ''
}

const enonceDe = (id: string) => epreuves.value.find(e => e.id === id)?.enonce ?? id

watch([filtreModule, filtreEtat], () => {
  page.value = 1
  chargerEpreuves().catch(e => { erreur.value = messageErreurAdminJeu(e) })
})
watch(page, () => {
  chargerEpreuves().catch(e => { erreur.value = messageErreurAdminJeu(e) })
})

onMounted(rafraichir)
</script>

<template>
  <div>
    <AdminPageHeader
      titre="Revue des épreuves"
      sous-titre="Faire proposer des épreuves à partir du contenu publié, puis les accepter, les corriger ou les rejeter"
    />

    <div v-if="message" class="alert alert-success mb-4 text-sm">{{ message }}</div>
    <div v-if="erreur" class="alert alert-error mb-4 text-sm" role="alert">{{ erreur }}</div>

    <div v-if="chargement" class="flex justify-center py-16">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <template v-else>
      <!-- 1. Proposer des épreuves -->
      <section class="mb-6">
        <h2 class="mb-3 text-lg font-bold">Proposer des épreuves</h2>
        <div class="grid gap-3 md:grid-cols-2 xl:grid-cols-4">
          <div v-for="groupe in formesParModule" :key="groupe.module.code" class="card bg-base-200">
            <div class="card-body gap-3 p-4">
              <h3 class="font-bold">{{ groupe.module.libelle }}</h3>

              <!-- Un module sans forme : on le DIT, on ne propose pas un bouton
                   qui ne ferait rien. -->
              <p v-if="groupe.formes.length === 0" class="text-sm text-base-content/70">
                Pas de dérivation pour ce module : ses épreuves se saisissent.
                <NuxtLink to="/admin/activites/epreuves/nouvelle" class="link">Saisir une épreuve</NuxtLink>
              </p>

              <template v-else>
                <ul class="space-y-1 text-sm">
                  <li v-for="f in groupe.formes" :key="f.forme" class="flex items-baseline justify-between gap-2">
                    <span>{{ f.libelle }}</span>
                    <span v-if="f.type_reponse !== 'choix'" class="badge badge-outline badge-xs">{{ LIBELLES_TYPE_REPONSE[f.type_reponse] }}</span>
                    <span class="shrink-0 tabular-nums" :class="restantes(f) > 0 ? 'font-bold text-success' : 'text-base-content/50'">
                      {{ restantes(f) > 0 ? `+${restantes(f)}` : 'à jour' }}
                    </span>
                  </li>
                </ul>
                <button
                  type="button"
                  class="btn btn-primary btn-sm mt-1"
                  :disabled="enCours || groupe.formes.every(f => restantes(f) === 0)"
                  @click="proposer(groupe.module.code)"
                >
                  <font-awesome-icon icon="wand-magic-sparkles" class="mr-1" />
                  Proposer
                </button>
              </template>
            </div>
          </div>
        </div>

        <div v-if="bilanDerivation" class="alert alert-info mt-3 text-sm">
          <span>
            <strong>{{ bilanDerivation.creees }}</strong> candidate{{ bilanDerivation.creees > 1 ? 's' : '' }}
            créée{{ bilanDerivation.creees > 1 ? 's' : '' }},
            {{ bilanDerivation.deja_proposees }} déjà proposée{{ bilanDerivation.deja_proposees > 1 ? 's' : '' }}
            <template v-if="bilanDerivation.sans_distracteurs > 0">,
              {{ bilanDerivation.sans_distracteurs }} sans assez de mauvaises réponses distinctes
            </template>.
          </span>
        </div>
      </section>

      <!-- 2. Passer en revue -->
      <section>
        <div class="mb-3 flex flex-wrap items-end gap-3">
          <h2 class="mr-auto text-lg font-bold">À revoir <span class="font-normal text-base-content/60">({{ total }})</span></h2>

          <div class="join">
            <button
              type="button"
              class="btn btn-sm join-item"
              :class="filtreEtat === 'candidate' && 'btn-active'"
              @click="filtreEtat = 'candidate'"
            >
              Candidates
            </button>
            <button
              type="button"
              class="btn btn-sm join-item"
              :class="filtreEtat === 'a_revoir' && 'btn-active'"
              @click="filtreEtat = 'a_revoir'"
            >
              Source modifiée
            </button>
          </div>

          <select v-model="filtreModule" class="select select-bordered select-sm" aria-label="Module">
            <option value="">Tous les modules</option>
            <option v-for="m in modules" :key="m.code" :value="m.code">{{ m.libelle }}</option>
          </select>
        </div>

        <!-- Refus du dernier lot : chaque épreuve écartée, avec sa raison -->
        <div v-if="bilanRevue?.refus.length" class="alert alert-warning mb-3 flex-col items-start text-sm">
          <strong>{{ bilanRevue.refus.length }} épreuve{{ bilanRevue.refus.length > 1 ? 's' : '' }} non traitée{{ bilanRevue.refus.length > 1 ? 's' : '' }} :</strong>
          <ul class="list-disc pl-5">
            <li v-for="refus in bilanRevue.refus" :key="refus.id">
              {{ refus.raison }} <span class="text-base-content/60">({{ enonceDe(refus.id) }})</span>
            </li>
          </ul>
        </div>

        <div v-if="epreuves.length === 0" class="card bg-base-100 p-10 text-center text-base-content/60 shadow-sm">
          Rien à revoir ici.
        </div>

        <template v-else>
          <div class="mb-2 flex flex-wrap items-center gap-3">
            <label class="flex cursor-pointer items-center gap-2 text-sm">
              <input type="checkbox" class="checkbox checkbox-sm" :checked="toutSelectionne" @change="basculerTout">
              Tout sélectionner sur cette page
            </label>
            <span class="text-sm text-base-content/60">{{ selection.size }} sélectionnée{{ selection.size > 1 ? 's' : '' }}</span>
            <button
              type="button"
              class="btn btn-success btn-sm ml-auto"
              :disabled="enCours || selection.size === 0"
              @click="decider('accepter', [...selection])"
            >
              <font-awesome-icon icon="check" class="mr-1" /> Accepter
            </button>
            <button
              type="button"
              class="btn btn-outline btn-error btn-sm"
              :disabled="enCours || selection.size === 0"
              @click="rejetOuvert = true"
            >
              <font-awesome-icon icon="xmark" class="mr-1" /> Rejeter
            </button>
          </div>

          <ul class="space-y-2">
            <li
              v-for="e in epreuves"
              :key="e.id"
              class="card bg-base-100 shadow-sm"
              :class="selection.has(e.id) && 'ring-2 ring-primary/40'"
            >
              <div class="card-body flex-row gap-4 p-4">
                <input
                  type="checkbox"
                  class="checkbox checkbox-sm mt-1"
                  :checked="selection.has(e.id)"
                  :aria-label="`Sélectionner : ${e.enonce}`"
                  @change="basculer(e.id)"
                >
                <div class="min-w-0 flex-1">
                  <div class="flex flex-wrap items-center gap-2 text-xs text-base-content/60">
                    <span class="badge badge-outline badge-sm">{{ e.module_code }}</span>
                    <span v-if="e.forme">{{ e.forme }}</span>
                    <span v-if="e.pays_nom">· {{ e.pays_nom }}</span>
                    <span v-if="e.source_etat === 'modifiee'" class="text-warning">· source modifiée depuis la dérivation</span>
                    <span v-if="e.source_etat === 'indisponible'" class="text-error">· source dépubliée</span>
                  </div>
                  <p class="mt-1 font-medium">{{ e.enonce }}</p>
                  <img
                    v-if="e.media_type === 'image' && e.media_url"
                    :src="urlMedia(e.media_url) ?? undefined"
                    alt=""
                    class="mt-2 h-16 rounded border border-base-300"
                  >
                  <AdminJeuApercuSolution
                    :type-reponse="e.type_reponse"
                    :propositions="e.propositions"
                    :bonne-reponse="e.bonne_reponse"
                    :pays-nom="e.reponse_pays_nom"
                    :elements="e.elements_attendus"
                    :paires="e.paires_attendues"
                  />
                  <p v-if="e.explication" class="mt-2 text-sm text-base-content/70">{{ e.explication }}</p>
                </div>
                <div class="flex shrink-0 flex-col gap-1">
                  <NuxtLink :to="`/admin/activites/epreuves/${e.id}`" class="btn btn-ghost btn-xs">
                    <font-awesome-icon icon="pen-to-square" class="mr-1" /> Corriger
                  </NuxtLink>
                  <button
                    type="button"
                    class="btn btn-ghost btn-xs text-success"
                    :disabled="enCours"
                    @click="decider('accepter', [e.id])"
                  >
                    <font-awesome-icon icon="check" class="mr-1" /> Accepter
                  </button>
                </div>
              </div>
            </li>
          </ul>

          <div v-if="totalPages > 1" class="mt-4 flex items-center justify-center gap-3">
            <button type="button" class="btn btn-sm" :disabled="page <= 1" @click="page -= 1">Précédent</button>
            <span class="text-sm">Page {{ page }} / {{ totalPages }}</span>
            <button type="button" class="btn btn-sm" :disabled="page >= totalPages" @click="page += 1">Suivant</button>
          </div>
        </template>
      </section>
    </template>

    <!-- Rejet : le motif est obligatoire, et la forme rejetée ne reviendra pas -->
    <AdminFormModal
      v-model:visible="rejetOuvert"
      titre="Rejeter la sélection"
      mode="edition"
      taille="md"
      @soumettre="confirmerRejet"
    >
      <p class="mb-3 text-sm text-base-content/70">
        {{ selection.size }} épreuve{{ selection.size > 1 ? 's' : '' }}. Une épreuve rejetée ne sera plus
        jamais proposée pour ce contenu et cette forme de question.
      </p>
      <label class="flex flex-col">
        <span class="label-text mb-1 font-medium">Motif du rejet *</span>
        <textarea
          v-model="motifRejet"
          rows="3"
          class="textarea textarea-bordered w-full"
          placeholder="Distracteurs trop proches, question ambiguë…"
          required
        />
      </label>
    </AdminFormModal>
  </div>
</template>
