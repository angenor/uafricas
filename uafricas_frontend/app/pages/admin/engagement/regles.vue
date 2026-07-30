<script setup lang="ts">
/**
 * Back-office — règles de points et paliers de popularité (daisyUI autorisé ici).
 *
 * Les niveaux ont leur écran dédié (`/admin/engagement/niveaux`) : leurs mutations
 * rebasculent tous les comptes et méritent leur propre retour d'information.
 */
import { computed, ref, onMounted } from 'vue'
import {
  useAdminEngagement,
  type AdminRegle,
  type AdminPalier,
  type AdminCategorie,
  type AdminActionDisponible,
} from '~/composables/useAdminEngagement'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const {
  listerRegles, listerActionsDisponibles, creerRegle, modifierRegle, supprimerRegle,
  listerCategories,
  listerPaliers, creerPalier, modifierPalier, desactiverPalier,
} = useAdminEngagement()

const regles = ref<AdminRegle[]>([])
const paliers = ref<AdminPalier[]>([])
const categories = ref<AdminCategorie[]>([])
const actionsDisponibles = ref<AdminActionDisponible[]>([])
const chargement = ref(true)
const message = ref('')
const erreur = ref('')

/** Familles de contenus auxquelles un palier peut être restreint (miroir du serveur). */
const FAMILLES = [
  'codimoi', 'factcheck', 'video', 'biblio_humaine',
  'chaine_tv', 'station_radio', 'programme_tele', 'programme_radio',
]

const nouveauPalier = ref<{ seuil_likes: number, points: number, type_objet: string }>({
  seuil_likes: 0, points: 0, type_objet: '',
})

const formulaireRegle = ref(false)
const nouvelleRegle = ref({
  type_action: '',
  libelle: '',
  points: 0,
  reputation_delta: 0,
  plafond_journalier: null as number | null,
  plafond_mensuel: null as number | null,
  seuil_declencheur: null as number | null,
  categorie_id: '' as string,
})

const rafraichir = async () => {
  chargement.value = true
  ;[regles.value, paliers.value, categories.value, actionsDisponibles.value] = await Promise.all([
    listerRegles(), listerPaliers(), listerCategories(), listerActionsDisponibles(),
  ])
  chargement.value = false
}

onMounted(rafraichir)

const notifier = (m: string) => {
  message.value = m
  erreur.value = ''
  setTimeout(() => { message.value = '' }, 2500)
}

/** Les 409/400 du serveur portent un message français explicite : on l'affiche tel quel. */
const signaler = (e: unknown) => {
  message.value = ''
  const data = (e as { data?: { error?: string } })?.data
  erreur.value = data?.error || 'L\'opération a échoué.'
}

// ── Règles ──────────────────────────────────────────────────────────────────

/** Les actions du catalogue pas encore couvertes par une règle. */
const actionsSansRegle = computed(() => actionsDisponibles.value.filter(a => !a.regle_existante))

/** Pré-remplit le formulaire depuis le catalogue : évite la faute de frappe sur la clé. */
const choisirAction = (typeAction: string) => {
  const a = actionsDisponibles.value.find(x => x.type_action === typeAction)
  if (a && !nouvelleRegle.value.libelle) nouvelleRegle.value.libelle = a.libelle_defaut
}

const enregistrerRegle = async (r: AdminRegle) => {
  try {
    await modifierRegle(r.id, {
      libelle: r.libelle,
      points: r.points,
      reputation_delta: r.reputation_delta,
      plafond_journalier: r.plafond_journalier,
      plafond_mensuel: r.plafond_mensuel,
      seuil_declencheur: r.seuil_declencheur,
      categorie_id: r.categorie_id,
      actif: r.actif,
    })
    notifier('Règle enregistrée')
  } catch (e) { signaler(e) }
}

const ajouterRegle = async () => {
  const f = nouvelleRegle.value
  if (!f.type_action.trim() || !f.libelle.trim()) {
    erreur.value = 'L\'action et le libellé sont obligatoires.'
    return
  }
  try {
    await creerRegle({
      type_action: f.type_action.trim(),
      libelle: f.libelle.trim(),
      points: f.points,
      reputation_delta: f.reputation_delta,
      plafond_journalier: f.plafond_journalier,
      plafond_mensuel: f.plafond_mensuel,
      seuil_declencheur: f.seuil_declencheur,
      categorie_id: f.categorie_id || null,
    })
    formulaireRegle.value = false
    nouvelleRegle.value = {
      type_action: '', libelle: '', points: 0, reputation_delta: 0,
      plafond_journalier: null, plafond_mensuel: null, seuil_declencheur: null, categorie_id: '',
    }
    await rafraichir()
    notifier('Règle créée')
  } catch (e) { signaler(e) }
}

const retirerRegle = async (r: AdminRegle) => {
  if (r.nombre_mouvements > 0) {
    erreur.value = 'Cette règle a déjà attribué des points : décochez « Actif » au lieu de la supprimer.'
    return
  }
  if (!confirm(`Supprimer définitivement la règle « ${r.libelle} » ?`)) return
  try {
    await supprimerRegle(r.id)
    await rafraichir()
    notifier('Règle supprimée')
  } catch (e) { signaler(e) }
}

// ── Paliers, groupés par famille ────────────────────────────────────────────

/** Les paliers globaux d'abord, puis un groupe par famille restreinte. */
const paliersGroupes = computed(() => {
  const groupes = new Map<string, AdminPalier[]>()
  for (const p of paliers.value) {
    const cle = p.type_objet ?? ''
    if (!groupes.has(cle)) groupes.set(cle, [])
    groupes.get(cle)!.push(p)
  }
  return [...groupes.entries()]
    .sort(([a], [b]) => (a === '' ? -1 : b === '' ? 1 : a.localeCompare(b)))
    .map(([famille, liste]) => ({ famille, liste }))
})

const enregistrerPalier = async (p: AdminPalier) => {
  try {
    await modifierPalier(p.id, { points: p.points, actif: p.actif })
    notifier('Palier enregistré')
  } catch (e) { signaler(e) }
}

const ajouterPalier = async () => {
  if (nouveauPalier.value.seuil_likes <= 0) {
    erreur.value = 'Le seuil de « j\'aime » doit être positif.'
    return
  }
  try {
    await creerPalier(
      nouveauPalier.value.seuil_likes,
      nouveauPalier.value.points,
      nouveauPalier.value.type_objet || null,
    )
    nouveauPalier.value = { seuil_likes: 0, points: 0, type_objet: '' }
    await rafraichir()
    notifier('Palier ajouté')
  } catch (e) { signaler(e) }
}

const retirerPalier = async (p: AdminPalier) => {
  try {
    await desactiverPalier(p.id)
    await rafraichir()
  } catch (e) { signaler(e) }
}
</script>

<template>
  <div class="p-6 space-y-8">
    <div class="flex flex-wrap items-center justify-between gap-3">
      <h1 class="text-2xl font-bold">Engagement — Barème</h1>
      <div class="flex flex-wrap gap-2">
        <NuxtLink to="/admin/engagement/categories" class="btn btn-sm btn-outline">
          <font-awesome-icon icon="fa-solid fa-layer-group" /> Catégories
        </NuxtLink>
        <NuxtLink to="/admin/engagement/niveaux" class="btn btn-sm btn-outline">
          <font-awesome-icon icon="fa-solid fa-medal" /> Niveaux
        </NuxtLink>
        <NuxtLink to="/admin/engagement/journal" class="btn btn-sm btn-outline">
          <font-awesome-icon icon="fa-solid fa-list" /> Journal des points
        </NuxtLink>
      </div>
    </div>

    <div v-if="message" class="alert alert-success py-2">{{ message }}</div>
    <div v-if="erreur" class="alert alert-error py-2">{{ erreur }}</div>
    <div v-if="chargement" class="loading loading-spinner" />

    <template v-else>
      <!-- ─── Règles ─── -->
      <section class="space-y-3">
        <div class="flex flex-wrap items-center justify-between gap-2">
          <h2 class="text-lg font-semibold">Règles de points</h2>
          <button class="btn btn-sm btn-primary" @click="formulaireRegle = !formulaireRegle">
            <font-awesome-icon :icon="formulaireRegle ? 'fa-solid fa-xmark' : 'fa-solid fa-plus'" />
            {{ formulaireRegle ? 'Annuler' : 'Nouvelle règle' }}
          </button>
        </div>

        <!--
          Piège de paramétrage n°1 : le moteur compare la SOMME DES POINTS du jour
          au plafond. Un administrateur qui saisit « 3 » en croyant limiter à 3
          actions plafonne en réalité à 3 points.
        -->
        <div class="alert alert-info py-2 text-sm">
          <font-awesome-icon icon="fa-solid fa-circle-info" />
          <span>
            Les plafonds sont exprimés <strong>en points, pas en nombre d'actions</strong>.
            « 3 bonus de 10 points par jour » se paramètre donc à <strong>30</strong>, pas à 3.
          </span>
        </div>

        <!-- Création : le catalogue d'actions d'abord -->
        <div v-if="formulaireRegle" class="card bg-base-200">
          <div class="card-body gap-3">
            <h3 class="font-semibold">Créer une règle</h3>

            <div v-if="actionsSansRegle.length > 0" class="space-y-1">
              <p class="text-sm">
                Actions <strong>branchées dans le code</strong> et pas encore couvertes par une règle :
              </p>
              <div class="flex flex-wrap gap-2">
                <button
                  v-for="a in actionsSansRegle"
                  :key="a.type_action"
                  type="button"
                  class="btn btn-xs btn-outline"
                  :title="`Module : ${a.module}`"
                  @click="nouvelleRegle.type_action = a.type_action; choisirAction(a.type_action)"
                >
                  {{ a.libelle_defaut }}
                </button>
              </div>
            </div>
            <p v-else class="text-sm opacity-70">
              Toutes les actions instrumentées par le code sont déjà couvertes par une règle.
            </p>

            <div class="grid grid-cols-1 gap-3 md:grid-cols-2 lg:grid-cols-4">
              <label class="form-control">
                <span class="label-text text-xs">Action (clé technique)</span>
                <input
                  v-model="nouvelleRegle.type_action"
                  class="input input-sm input-bordered font-mono"
                  placeholder="ex. contribution_validee"
                >
              </label>
              <label class="form-control">
                <span class="label-text text-xs">Libellé affiché</span>
                <input v-model="nouvelleRegle.libelle" class="input input-sm input-bordered">
              </label>
              <label class="form-control">
                <span class="label-text text-xs">Catégorie</span>
                <select v-model="nouvelleRegle.categorie_id" class="select select-sm select-bordered">
                  <option value="">— aucune —</option>
                  <option v-for="c in categories" :key="c.id" :value="c.id">{{ c.libelle }}</option>
                </select>
              </label>
              <label class="form-control">
                <span class="label-text text-xs">Points (négatif = malus)</span>
                <input v-model.number="nouvelleRegle.points" type="number" class="input input-sm input-bordered">
              </label>
              <label class="form-control">
                <span class="label-text text-xs">Réputation</span>
                <input v-model.number="nouvelleRegle.reputation_delta" type="number" class="input input-sm input-bordered">
              </label>
              <label class="form-control">
                <span class="label-text text-xs">Plafond / jour (points)</span>
                <input v-model.number="nouvelleRegle.plafond_journalier" type="number" class="input input-sm input-bordered" placeholder="∞">
              </label>
              <label class="form-control">
                <span class="label-text text-xs">Plafond / mois (points)</span>
                <input v-model.number="nouvelleRegle.plafond_mensuel" type="number" class="input input-sm input-bordered" placeholder="∞">
              </label>
              <label class="form-control">
                <span class="label-text text-xs">Seuil déclencheur (occurrences)</span>
                <input v-model.number="nouvelleRegle.seuil_declencheur" type="number" class="input input-sm input-bordered" placeholder="immédiat">
              </label>
            </div>

            <div class="card-actions justify-end">
              <button class="btn btn-sm btn-success" @click="ajouterRegle">Créer la règle</button>
            </div>
          </div>
        </div>

        <div class="overflow-x-auto">
          <table class="table table-zebra table-sm">
            <thead>
              <tr>
                <th>Action</th><th>Libellé</th><th>Catégorie</th><th>Points</th><th>Réputation</th>
                <th>Plafond / jour</th><th>Plafond / mois</th><th>Seuil décl.</th>
                <th>Mouvements</th><th>Actif</th><th />
              </tr>
            </thead>
            <tbody>
              <tr v-for="r in regles" :key="r.id">
                <td class="font-mono text-xs">
                  {{ r.type_action }}
                  <!--
                    Sans cette mention, une règle créée pour une action que le code
                    n'émet pas resterait silencieusement stérile : c'est le bug le
                    plus coûteux à diagnostiquer du paramétrage.
                  -->
                  <span
                    v-if="!r.instrumentee"
                    class="badge badge-warning badge-sm mt-1 block whitespace-normal text-left"
                  >
                    non instrumentée — aucun point ne sera attribué tant que le code n'émet pas cette action
                  </span>
                </td>
                <td><input v-model="r.libelle" class="input input-sm input-bordered w-44"></td>
                <td>
                  <select v-model="r.categorie_id" class="select select-sm select-bordered w-36">
                    <option :value="null">— aucune —</option>
                    <option v-for="c in categories" :key="c.id" :value="c.id">{{ c.libelle }}</option>
                  </select>
                </td>
                <td><input v-model.number="r.points" type="number" class="input input-sm input-bordered w-20"></td>
                <td><input v-model.number="r.reputation_delta" type="number" class="input input-sm input-bordered w-20"></td>
                <td><input v-model.number="r.plafond_journalier" type="number" class="input input-sm input-bordered w-24" placeholder="∞"></td>
                <td><input v-model.number="r.plafond_mensuel" type="number" class="input input-sm input-bordered w-24" placeholder="∞"></td>
                <td><input v-model.number="r.seuil_declencheur" type="number" class="input input-sm input-bordered w-20" placeholder="—"></td>
                <td class="text-xs">
                  {{ r.nombre_mouvements }}
                  <span v-if="r.actif && r.instrumentee && r.nombre_mouvements === 0" class="block text-warning">
                    jamais déclenchée
                  </span>
                </td>
                <td><input v-model="r.actif" type="checkbox" class="toggle toggle-sm"></td>
                <td class="flex gap-1">
                  <button class="btn btn-xs btn-primary" @click="enregistrerRegle(r)">Enregistrer</button>
                  <button
                    class="btn btn-xs btn-error btn-outline"
                    :disabled="r.nombre_mouvements > 0"
                    :title="r.nombre_mouvements > 0
                      ? 'Règle déjà utilisée : désactivez-la plutôt'
                      : 'Supprimer'"
                    @click="retirerRegle(r)"
                  >
                    <font-awesome-icon icon="fa-solid fa-trash" />
                  </button>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </section>

      <!-- ─── Paliers de popularité, groupés par famille ─── -->
      <section class="space-y-3">
        <h2 class="text-lg font-semibold">Paliers de popularité (« j'aime » → points)</h2>

        <!--
          Sans ce rappel, un administrateur croira que les paliers d'une famille
          S'AJOUTENT aux globaux, alors qu'ils les remplacent.
        -->
        <div class="alert alert-warning py-2 text-sm">
          <font-awesome-icon icon="fa-solid fa-triangle-exclamation" />
          <span>
            Les paliers d'une famille <strong>remplacent</strong> les paliers globaux pour cette
            famille — ils ne s'y ajoutent pas. Une famille sans palier propre suit les globaux.
          </span>
        </div>

        <div v-for="groupe in paliersGroupes" :key="groupe.famille || 'global'" class="card bg-base-200">
          <div class="card-body gap-2 py-4">
            <h3 class="text-sm font-semibold">
              <font-awesome-icon :icon="groupe.famille ? 'fa-solid fa-filter' : 'fa-solid fa-globe'" />
              {{ groupe.famille ? `Famille « ${groupe.famille} »` : 'Paliers globaux (toutes familles)' }}
            </h3>
            <div class="overflow-x-auto">
              <table class="table table-sm">
                <thead><tr><th>Seuil (j'aime)</th><th>Points</th><th>Actif</th><th /></tr></thead>
                <tbody>
                  <tr v-for="p in groupe.liste" :key="p.id">
                    <td>{{ p.seuil_likes }}</td>
                    <td><input v-model.number="p.points" type="number" class="input input-sm input-bordered w-24"></td>
                    <td><input v-model="p.actif" type="checkbox" class="toggle toggle-sm"></td>
                    <td class="flex gap-2">
                      <button class="btn btn-xs btn-primary" @click="enregistrerPalier(p)">Enregistrer</button>
                      <button class="btn btn-xs btn-error btn-outline" @click="retirerPalier(p)">Désactiver</button>
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>

        <div class="card bg-base-100 border border-base-300">
          <div class="card-body flex-row flex-wrap items-end gap-3 py-4">
            <label class="form-control">
              <span class="label-text text-xs">Seuil (j'aime)</span>
              <input v-model.number="nouveauPalier.seuil_likes" type="number" class="input input-sm input-bordered w-28" placeholder="ex. 2000">
            </label>
            <label class="form-control">
              <span class="label-text text-xs">Points</span>
              <input v-model.number="nouveauPalier.points" type="number" class="input input-sm input-bordered w-24" placeholder="ex. 80">
            </label>
            <label class="form-control">
              <span class="label-text text-xs">Famille (vide = global)</span>
              <select v-model="nouveauPalier.type_objet" class="select select-sm select-bordered w-44">
                <option value="">Global (toutes familles)</option>
                <option v-for="f in FAMILLES" :key="f" :value="f">{{ f }}</option>
              </select>
            </label>
            <button class="btn btn-sm btn-success" @click="ajouterPalier">
              <font-awesome-icon icon="fa-solid fa-plus" /> Ajouter le palier
            </button>
          </div>
        </div>
      </section>
    </template>
  </div>
</template>
