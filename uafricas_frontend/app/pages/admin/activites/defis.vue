<script setup lang="ts">
/**
 * Défis du jour et de la semaine (feature 013), côté administration.
 *
 * Un défi existe sans que personne ait à le préparer : à défaut de
 * programmation, il est composé automatiquement à l'ouverture de sa période.
 * Cet écran sert à programmer une période À VENIR ; le défi en cours est figé,
 * des membres l'ont peut-être déjà joué.
 *
 * Les périodes sont en temps universel : le « jour » d'un défi va de minuit à
 * minuit UTC, la semaine commence le lundi UTC.
 */
import {
  messageErreurAdminJeu,
  type DefiAdminAPI,
  type EpreuveAdminAPI,
  type ModuleAdminAPI,
} from '~/composables/useAdminJeu'
import type { ApiResponse, PaginatedResponse } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { listerDefis, programmerDefi, deprogrammerDefi, listerModules } = useAdminJeu()
const { adminFetch } = useAdmin()

const TAILLES: Record<'jour' | 'semaine', number> = reactive({ jour: 5, semaine: 15 })

const defis = ref<DefiAdminAPI[]>([])
const modules = ref<ModuleAdminAPI[]>([])
const chargement = ref(true)
const enCours = ref(false)
const erreur = ref('')
const message = ref('')

// ── Formulaire de programmation ──────────────────────────────────────────────
const ouvert = ref(false)
const periodicite = ref<'jour' | 'semaine'>('jour')
const date = ref('')
const titre = ref('')
const moduleFiltre = ref('')
const recherche = ref('')
const candidates = ref<EpreuveAdminAPI[]>([])
const selection = ref<EpreuveAdminAPI[]>([])

const taille = computed(() => TAILLES[periodicite.value])
const complet = computed(() => selection.value.length === taille.value)

/** Première période programmable : demain, ou le lundi qui vient. */
const premiereDate = (p: 'jour' | 'semaine'): string => {
  const d = new Date()
  d.setUTCHours(0, 0, 0, 0)
  if (p === 'jour') d.setUTCDate(d.getUTCDate() + 1)
  else d.setUTCDate(d.getUTCDate() + ((8 - d.getUTCDay()) % 7 || 7))
  return d.toISOString().slice(0, 10)
}

const dateLisible = (iso: string) =>
  new Date(`${iso}T00:00:00Z`).toLocaleDateString('fr-FR', {
    weekday: 'short', day: 'numeric', month: 'short', year: 'numeric', timeZone: 'UTC',
  })

const notifier = (texte: string) => {
  message.value = texte
  setTimeout(() => { if (message.value === texte) message.value = '' }, 5000)
}

const charger = async () => {
  erreur.value = ''
  try {
    const [d, m, regles] = await Promise.all([
      listerDefis(),
      listerModules(),
      // Les tailles viennent des règles du jeu ; à défaut, les valeurs d'origine.
      adminFetch<ApiResponse<{ taille_defi_jour: number, taille_defi_semaine: number }>>(
        '/api/admin/jeu/regles',
      ).catch(() => null),
    ])
    defis.value = d
    modules.value = m
    if (regles?.data) {
      TAILLES.jour = regles.data.taille_defi_jour
      TAILLES.semaine = regles.data.taille_defi_semaine
    }
  }
  catch (e) {
    erreur.value = messageErreurAdminJeu(e, 'Impossible de charger les défis.')
  }
  finally {
    chargement.value = false
  }
}

const chargerCandidates = async () => {
  try {
    const reponse = await adminFetch<ApiResponse<PaginatedResponse<EpreuveAdminAPI>>>(
      '/api/admin/jeu/epreuves',
      { params: {
        etat: 'jouable', module: moduleFiltre.value, recherche: recherche.value,
        par_page: 50, tri_par: 'nombre_servie', tri_dir: 'asc',
      } },
    )
    // Une épreuve dont la source n'est plus publiée n'est pas servable : le
    // serveur refuserait la série, autant ne pas la proposer.
    candidates.value = (reponse.data?.data ?? []).filter(e => e.source_etat !== 'indisponible')
  }
  catch (e) {
    erreur.value = messageErreurAdminJeu(e)
  }
}

const ouvrir = (p: 'jour' | 'semaine' = 'jour') => {
  periodicite.value = p
  date.value = premiereDate(p)
  titre.value = ''
  selection.value = []
  ouvert.value = true
  chargerCandidates()
}

const estChoisie = (id: string) => selection.value.some(e => e.id === id)

const basculer = (epreuve: EpreuveAdminAPI) => {
  if (estChoisie(epreuve.id)) selection.value = selection.value.filter(e => e.id !== epreuve.id)
  else if (!complet.value) selection.value = [...selection.value, epreuve]
}

/** Complète la série au hasard parmi les épreuves affichées. */
const completerAuHasard = () => {
  const libres = candidates.value.filter(e => !estChoisie(e.id))
  for (let i = libres.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1))
    ;[libres[i], libres[j]] = [libres[j]!, libres[i]!]
  }
  selection.value = [...selection.value, ...libres.slice(0, taille.value - selection.value.length)]
}

const enregistrer = async () => {
  erreur.value = ''
  enCours.value = true
  try {
    await programmerDefi(periodicite.value, date.value, {
      titre: titre.value.trim() || null,
      module: moduleFiltre.value || null,
      epreuve_ids: selection.value.map(e => e.id),
    })
    ouvert.value = false
    notifier(`Défi du ${dateLisible(date.value)} programmé.`)
    await charger()
  }
  catch (e) {
    erreur.value = messageErreurAdminJeu(e)
  }
  finally {
    enCours.value = false
  }
}

const deprogrammer = async (defi: DefiAdminAPI) => {
  if (!confirm(`Retirer la programmation du ${dateLisible(defi.periode_debut)} ? Le défi de cette période sera composé automatiquement.`)) return
  erreur.value = ''
  try {
    await deprogrammerDefi(defi.periodicite, defi.periode_debut)
    notifier('Programmation retirée.')
    await charger()
  }
  catch (e) {
    erreur.value = messageErreurAdminJeu(e)
  }
}

watch(periodicite, (p) => {
  date.value = premiereDate(p)
  selection.value = selection.value.slice(0, TAILLES[p])
})
watch([moduleFiltre], chargerCandidates)

onMounted(charger)
</script>

<template>
  <div>
    <AdminPageHeader
      titre="Défis"
      sous-titre="Le défi du jour et celui de la semaine : la même série pour tous, un seul essai"
    >
      <template #actions>
        <button type="button" class="btn btn-primary btn-sm" @click="ouvrir('jour')">
          <font-awesome-icon icon="plus" class="mr-1" /> Programmer un défi
        </button>
      </template>
    </AdminPageHeader>

    <div v-if="message" class="alert alert-success mb-4 text-sm">{{ message }}</div>
    <div v-if="erreur" class="alert alert-error mb-4 text-sm" role="alert">{{ erreur }}</div>

    <div class="alert mb-4 text-sm">
      <font-awesome-icon icon="circle-info" />
      <span>
        Sans programmation, le défi d'une période est composé automatiquement à son ouverture.
        Les périodes sont en temps universel (UTC).
      </span>
    </div>

    <!-- Programmation -->
    <section v-if="ouvert" class="card mb-6 bg-base-200">
      <form class="card-body gap-4" @submit.prevent="enregistrer">
        <h2 class="text-lg font-bold">Programmer un défi</h2>

        <div class="grid gap-4 md:grid-cols-4">
          <label class="flex flex-col">
            <span class="label-text mb-1 font-medium">Périodicité</span>
            <select v-model="periodicite" class="select select-bordered select-sm w-full">
              <option value="jour">Défi du jour ({{ TAILLES.jour }} épreuves)</option>
              <option value="semaine">Défi de la semaine ({{ TAILLES.semaine }} épreuves)</option>
            </select>
          </label>
          <label class="flex flex-col">
            <span class="label-text mb-1 font-medium">{{ periodicite === 'jour' ? 'Jour' : 'Lundi de la semaine' }}</span>
            <input
              v-model="date"
              type="date"
              class="input input-bordered input-sm w-full"
              :min="premiereDate(periodicite)"
              :step="periodicite === 'semaine' ? 7 : 1"
              required
            >
          </label>
          <label class="flex flex-col md:col-span-2">
            <span class="label-text mb-1 font-medium">Titre (facultatif)</span>
            <input
              v-model="titre"
              type="text"
              maxlength="120"
              class="input input-bordered input-sm w-full"
              placeholder="Semaine des capitales"
            >
          </label>
        </div>

        <div class="grid gap-4 lg:grid-cols-2">
          <!-- Vivier -->
          <div>
            <div class="mb-2 flex flex-wrap items-end gap-2">
              <select v-model="moduleFiltre" class="select select-bordered select-sm" aria-label="Module">
                <option value="">Tous les modules</option>
                <option v-for="m in modules" :key="m.code" :value="m.code">{{ m.libelle }}</option>
              </select>
              <input
                v-model="recherche"
                type="search"
                class="input input-bordered input-sm flex-1"
                placeholder="Chercher dans les énoncés…"
                @keydown.enter.prevent="chargerCandidates"
              >
              <button type="button" class="btn btn-sm" @click="chargerCandidates">Chercher</button>
            </div>
            <ul class="max-h-80 space-y-1 overflow-y-auto rounded-lg bg-base-100 p-2">
              <li v-if="candidates.length === 0" class="p-3 text-sm text-base-content/60">
                Aucune épreuve jouable ne correspond.
              </li>
              <li v-for="e in candidates" :key="e.id">
                <label
                  class="flex cursor-pointer items-start gap-2 rounded p-2 text-sm hover:bg-base-200"
                  :class="!estChoisie(e.id) && complet && 'cursor-not-allowed opacity-50'"
                >
                  <input
                    type="checkbox"
                    class="checkbox checkbox-sm mt-0.5"
                    :checked="estChoisie(e.id)"
                    :disabled="!estChoisie(e.id) && complet"
                    @change="basculer(e)"
                  >
                  <span class="min-w-0 flex-1">
                    {{ e.enonce }}
                    <span class="block text-xs text-base-content/50">{{ e.module_code }} · servie {{ e.nombre_servie }} fois</span>
                  </span>
                </label>
              </li>
            </ul>
          </div>

          <!-- Série -->
          <div>
            <div class="mb-2 flex items-center justify-between gap-2">
              <span class="text-sm font-medium" :class="complet ? 'text-success' : ''">
                Série : {{ selection.length }} / {{ taille }}
              </span>
              <button
                type="button"
                class="btn btn-ghost btn-xs"
                :disabled="complet || candidates.length === 0"
                @click="completerAuHasard"
              >
                <font-awesome-icon icon="shuffle" class="mr-1" /> Compléter au hasard
              </button>
            </div>
            <ol class="max-h-80 space-y-1 overflow-y-auto rounded-lg bg-base-100 p-2">
              <li v-if="selection.length === 0" class="p-3 text-sm text-base-content/60">
                Cochez des épreuves à gauche : elles seront servies dans cet ordre.
              </li>
              <li v-for="(e, index) in selection" :key="e.id" class="flex items-start gap-2 rounded p-2 text-sm">
                <span class="w-5 shrink-0 font-bold text-base-content/50">{{ index + 1 }}</span>
                <span class="min-w-0 flex-1">{{ e.enonce }}</span>
                <button
                  type="button"
                  class="btn btn-ghost btn-xs"
                  :aria-label="`Retirer : ${e.enonce}`"
                  @click="basculer(e)"
                >
                  <font-awesome-icon icon="xmark" />
                </button>
              </li>
            </ol>
          </div>
        </div>

        <div class="flex justify-end gap-2">
          <button type="button" class="btn btn-ghost btn-sm" @click="ouvert = false">Annuler</button>
          <button type="submit" class="btn btn-primary btn-sm" :disabled="!complet || !date || enCours">
            Programmer
          </button>
        </div>
      </form>
    </section>

    <div v-if="chargement" class="flex justify-center py-16">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <div v-else class="card overflow-x-auto bg-base-100 shadow-sm">
      <table class="table table-zebra table-sm">
        <thead>
          <tr>
            <th>Période</th>
            <th>Défi</th>
            <th>Titre</th>
            <th>Origine</th>
            <th class="text-center">Épreuves</th>
            <th class="text-center">Participants</th>
            <th class="text-center">Terminés</th>
            <th />
          </tr>
        </thead>
        <tbody>
          <tr v-if="defis.length === 0">
            <td colspan="8" class="py-8 text-center text-base-content/60">Aucun défi pour le moment.</td>
          </tr>
          <tr v-for="d in defis" :key="d.id">
            <td class="whitespace-nowrap">{{ dateLisible(d.periode_debut) }}</td>
            <td>{{ d.periodicite === 'jour' ? 'Jour' : 'Semaine' }}</td>
            <td>{{ d.titre || '—' }}</td>
            <td>
              <span class="badge badge-sm" :class="d.origine === 'programme' ? 'badge-primary' : 'badge-ghost'">
                {{ d.origine === 'programme' ? 'Programmé' : 'Automatique' }}
              </span>
              <span v-if="d.a_venir" class="badge badge-outline badge-sm ml-1">À venir</span>
            </td>
            <td class="text-center tabular-nums">{{ d.nombre_epreuves }}</td>
            <td class="text-center tabular-nums">{{ d.participants }}</td>
            <td class="text-center tabular-nums">{{ d.termines }}</td>
            <td class="text-right">
              <button
                v-if="d.a_venir"
                type="button"
                class="btn btn-ghost btn-xs text-error"
                @click="deprogrammer(d)"
              >
                Retirer
              </button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>
