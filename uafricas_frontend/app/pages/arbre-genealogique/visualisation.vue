<script setup lang="ts">
import { ref, onMounted, computed, watch } from 'vue'
import { useArbreGenealogique } from '~/composables/useArbreGenealogique'
import { useLayoutArbre, type NoeudArbre, type InfoIncompletude } from '~/composables/useLayoutArbre'
import { suggererTypeLien } from '~/mocks/arbre-genealogique'
import type { PersonneNoeud, LienArbreResponse, ModeVue, ModePanneau, ContexteAjout, CreerPersonneForm, TypeLien, Genre } from '~/mocks/arbre-genealogique'
import type { Node, Edge } from '@vue-flow/core'
import ArbreGraphe from '~/components/arbre-genealogique/ArbreGraphe.vue'
import BarreOutils from '~/components/arbre-genealogique/BarreOutils.vue'
import PanneauPersonne from '~/components/arbre-genealogique/PanneauPersonne.vue'
import AssistantAjoutPersonne from '~/components/arbre-genealogique/AssistantAjoutPersonne.vue'
import AssistantConversationnel from '~/components/arbre-genealogique/AssistantConversationnel.vue'

definePageMeta({
  middleware: 'auth',
  layout: 'default',
})

const { obtenirArbreComplet, creerPersonne, creerLien, modifierPersonne, supprimerPersonne } = useArbreGenealogique()
const { calculerLayout, calculerIncompletude, compterBranchesIncompletes } = useLayoutArbre()

// ─── État arbre ──────────────────────────────────────────────────────

const chargement = ref(true)
const erreur = ref<string | null>(null)
const personnes = ref<PersonneNoeud[]>([])
const liens = ref<LienArbreResponse[]>([])
const nodes = ref<Node[]>([])
const edges = ref<Edge[]>([])
const graphe = ref<Map<string, NoeudArbre>>(new Map())

const centreId = ref<string | null>(null)
const selectedId = ref<string | null>(null)
const mode = ref<ModeVue>('complet')
const personneCourante = ref<NoeudArbre | null>(null)

// ─── État panneau ────────────────────────────────────────────────────

const panneauOuvert = ref(false)
const modePanneau = ref<ModePanneau>('fiche')
const contexteAjout = ref<ContexteAjout | null>(null)
const mutationEnCours = ref(false)
const erreurMutation = ref<string | null>(null)
const afficherWizard = ref(false)
const wizardTypeLien = ref<'pere' | 'mere' | 'parent' | 'conjoint' | 'enfant' | undefined>(undefined)
const wizardTypeAction = ref<'parent' | 'enfant' | 'conjoint' | undefined>(undefined)
const wizardPersonneLiee = ref<{ id: string; nom: string; prenoms?: string } | undefined>(undefined)

// ─── État incomplétude ───────────────────────────────────────────────

const incompletude = ref<Map<string, InfoIncompletude>>(new Map())
const nbBranchesIncompletes = ref(0)

const arbreGrapheRef = ref<{ fitView: (opts?: object) => void; setCenter: (x: number, y: number, opts?: object) => void } | null>(null)

const estVide = computed(() => personnes.value.length === 0 && !chargement.value)

// ─── Assistant conversationnel ───────────────────────────────────────────

const assistantOuvert = ref(false)
const clicSeq = ref(0)
const cibleAssistant = ref<{ rattId: string; label: string; genre: Genre; seq: number } | null>(null)

// Ouvre automatiquement l'assistant lorsque l'arbre est vide (première visite).
watch(estVide, (vide) => {
  if (vide) assistantOuvert.value = true
})

// La largeur du canevas change quand l'assistant s'ouvre/se ferme : on recadre.
watch(assistantOuvert, () => {
  nextTick(() => {
    setTimeout(() => arbreGrapheRef.value?.fitView({ duration: 400 }), 100)
  })
})

// ─── Chargement ──────────────────────────────────────────────────────

const charger = async () => {
  chargement.value = true
  erreur.value = null
  try {
    const reponse = await obtenirArbreComplet()
    if (reponse.success && reponse.data) {
      personnes.value = reponse.data.personnes
      liens.value = reponse.data.liens
      if (personnes.value.length > 0) {
        // Choisir comme centre la personne la plus connectée
        if (!centreId.value || !personnes.value.find(p => p.rattachement_id === centreId.value)) {
          const nbLiensParRatt = new Map<string, number>()
          for (const p of personnes.value) nbLiensParRatt.set(p.rattachement_id, 0)
          for (const l of liens.value) {
            nbLiensParRatt.set(l.rattachement_source_id, (nbLiensParRatt.get(l.rattachement_source_id) ?? 0) + 1)
            nbLiensParRatt.set(l.rattachement_cible_id, (nbLiensParRatt.get(l.rattachement_cible_id) ?? 0) + 1)
          }
          let meilleurId = personnes.value[0].rattachement_id
          let maxLiens = 0
          for (const [id, nb] of nbLiensParRatt) {
            if (nb > maxLiens) { maxLiens = nb; meilleurId = id }
          }
          centreId.value = meilleurId
        }
        recalculer()
      }
    }
  } catch (e: any) {
    erreur.value = e?.data?.error || 'Erreur lors du chargement de l\'arbre'
  } finally {
    chargement.value = false
  }
}

const recalculer = () => {
  if (!centreId.value || personnes.value.length === 0) return
  const resultat = calculerLayout(personnes.value, liens.value, centreId.value, mode.value)
  nodes.value = resultat.nodes
  edges.value = resultat.edges
  graphe.value = resultat.graphe

  // Incomplétude
  incompletude.value = calculerIncompletude(resultat.graphe, liens.value)
  nbBranchesIncompletes.value = compterBranchesIncompletes(resultat.graphe, liens.value)

  // Injecter incomplétude dans les données des nœuds
  for (const node of nodes.value) {
    const info = incompletude.value.get(node.id)
    if (info) {
      node.data = { ...node.data, incompletude: info }
    }
  }
}

const rechargerArbre = async () => {
  await charger()
  nextTick(() => {
    arbreGrapheRef.value?.fitView({ duration: 500 })
  })
}

const reessayer = () => charger()

// ─── Interactions arbre ──────────────────────────────────────────────

const surClicNoeud = (nodeId: string) => {
  selectedId.value = nodeId
  centreId.value = nodeId
  const noeud = graphe.value.get(nodeId) || null
  personneCourante.value = noeud
  recalculer()

  // Clic sur une personne ⇒ interaction contextuelle dans l'assistant (chat).
  if (noeud) {
    const label = noeud.prenoms ? `${noeud.prenoms} ${noeud.nom}` : noeud.nom
    assistantOuvert.value = true
    clicSeq.value++
    cibleAssistant.value = { rattId: noeud.id, label, genre: noeud.genre as Genre, seq: clicSeq.value }
  }
}

const surClicFond = () => {
  panneauOuvert.value = false
  selectedId.value = null
  personneCourante.value = null
  modePanneau.value = 'fiche'
}

const surChangementMode = (nouveauMode: ModeVue) => {
  mode.value = nouveauMode
  recalculer()
  nextTick(() => {
    arbreGrapheRef.value?.fitView({ duration: 500 })
  })
}

const surResetVue = () => {
  arbreGrapheRef.value?.fitView({ duration: 500 })
}

const fermerPanneau = () => {
  panneauOuvert.value = false
  modePanneau.value = 'fiche'
}

// ─── Actions édition ─────────────────────────────────────────────────

const surActionAjouter = (typeAction: 'parent' | 'enfant' | 'conjoint') => {
  if (!personneCourante.value) return

  // Déterminer les types de parents existants
  const typesParents: TypeLien[] = []
  for (const lien of liens.value) {
    if (lien.rattachement_cible_id === personneCourante.value.id && lien.type_lien !== 'conjoint') {
      typesParents.push(lien.type_lien as TypeLien)
    }
  }

  const typeLien = suggererTypeLien(typeAction, personneCourante.value.genre, typesParents)

  contexteAjout.value = {
    personneSourceId: personneCourante.value.id,
    personneSourceNom: personneCourante.value.prenoms
      ? `${personneCourante.value.prenoms} ${personneCourante.value.nom}`
      : personneCourante.value.nom,
    typeAction,
    typeLienSuggere: typeLien,
  }

  // Ouvrir le wizard avec le contexte de lien
  wizardTypeLien.value = typeLien as 'pere' | 'mere' | 'parent' | 'conjoint' | 'enfant'
  wizardTypeAction.value = typeAction
  wizardPersonneLiee.value = {
    id: personneCourante.value.personne_id,
    nom: personneCourante.value.nom,
    prenoms: personneCourante.value.prenoms,
  }
  afficherWizard.value = true
  erreurMutation.value = null
}

const surActionModifier = () => {
  modePanneau.value = 'modifier'
  erreurMutation.value = null
}

const surRetourFiche = () => {
  modePanneau.value = 'fiche'
  contexteAjout.value = null
  erreurMutation.value = null
}

// ─── Flux ajout personne ─────────────────────────────────────────────

const surPersonneAjoutee = async (form: CreerPersonneForm, typeLien: TypeLien) => {
  if (!personneCourante.value || !contexteAjout.value) return
  mutationEnCours.value = true
  erreurMutation.value = null

  try {
    // Détection homonymes
    const nomComplet = `${form.prenoms ?? ''} ${form.nom}`.trim().toLowerCase()
    for (const noeud of graphe.value.values()) {
      const nomExistant = `${noeud.prenoms ?? ''} ${noeud.nom}`.trim().toLowerCase()
      if (nomExistant === nomComplet) {
        // Avertissement non-bloquant (on continue quand même)
        console.warn(`Homonyme détecté: ${form.prenoms} ${form.nom}`)
        break
      }
    }

    // 1. Créer la personne
    const reponseCreation = await creerPersonne(form)
    if (!reponseCreation.success || !reponseCreation.data) {
      erreurMutation.value = reponseCreation.error || 'Erreur lors de la création'
      return
    }

    const nouveauRattachementId = reponseCreation.data.rattachement_id
    const sourceId = personneCourante.value.id

    // 2. Créer le lien — orientation selon le type d'action
    let lienSource: string
    let lienCible: string

    if (contexteAjout.value.typeAction === 'parent') {
      // Nouvelle personne = parent de la personne courante
      lienSource = nouveauRattachementId
      lienCible = sourceId
    } else if (contexteAjout.value.typeAction === 'enfant') {
      // Personne courante = parent du nouvel enfant
      lienSource = sourceId
      lienCible = nouveauRattachementId
    } else {
      // Conjoint
      lienSource = sourceId
      lienCible = nouveauRattachementId
    }

    const reponseLien = await creerLien({
      rattachement_source_id: lienSource,
      rattachement_cible_id: lienCible,
      type_lien: typeLien,
    })

    if (!reponseLien.success) {
      // Rollback : supprimer la personne créée
      try {
        await supprimerPersonne(reponseCreation.data.personne.id)
      } catch {}

      // Mapper les erreurs backend
      const errBackend = reponseLien.error || ''
      if (errBackend.includes('circulaire') || errBackend.includes('cycle')) {
        erreurMutation.value = 'Ce lien créerait un cycle dans l\'arbre (une personne ne peut pas être son propre ancêtre)'
      } else if (errBackend.includes('existe déjà')) {
        erreurMutation.value = 'Un lien de ce type existe déjà entre ces deux personnes'
      } else {
        erreurMutation.value = errBackend || 'Erreur lors de la création du lien'
      }
      return
    }

    // 3. Recharger l'arbre
    await rechargerArbre()
    modePanneau.value = 'fiche'
    contexteAjout.value = null

    // Sélectionner la nouvelle personne
    const nouveauNoeud = graphe.value.get(nouveauRattachementId)
    if (nouveauNoeud) {
      personneCourante.value = nouveauNoeud
      selectedId.value = nouveauRattachementId
    }
  } catch (e: any) {
    erreurMutation.value = e?.data?.error || 'Connexion perdue. Veuillez réessayer.'
  } finally {
    mutationEnCours.value = false
  }
}

// ─── Flux wizard ajout ──────────────────────────────────────────────

const surWizardSubmit = async (form: CreerPersonneForm, typeLien?: TypeLien) => {
  if (!contexteAjout.value) return
  const lienFinal = typeLien ?? contexteAjout.value.typeLienSuggere
  await surPersonneAjoutee(form, lienFinal)
  afficherWizard.value = false
  wizardTypeLien.value = undefined
  wizardTypeAction.value = undefined
  wizardPersonneLiee.value = undefined
}

const surWizardAnnuler = () => {
  afficherWizard.value = false
  wizardTypeLien.value = undefined
  wizardTypeAction.value = undefined
  wizardPersonneLiee.value = undefined
  contexteAjout.value = null
}

const surWizardFormulaireClassique = () => {
  afficherWizard.value = false
  modePanneau.value = 'ajout'
}

// ─── Flux modification ───────────────────────────────────────────────

const surPersonneModifiee = async (form: CreerPersonneForm) => {
  if (!personneCourante.value) return
  mutationEnCours.value = true
  erreurMutation.value = null

  try {
    const reponse = await modifierPersonne(personneCourante.value.personne_id, form)
    if (!reponse.success) {
      erreurMutation.value = reponse.error || 'Erreur lors de la modification'
      return
    }

    await rechargerArbre()
    modePanneau.value = 'fiche'

    // Mettre à jour la personne courante
    const noeudMaj = graphe.value.get(personneCourante.value.id)
    if (noeudMaj) personneCourante.value = noeudMaj
  } catch (e: any) {
    erreurMutation.value = e?.data?.error || 'Connexion perdue. Veuillez réessayer.'
  } finally {
    mutationEnCours.value = false
  }
}

// ─── Flux suppression ────────────────────────────────────────────────

const surPersonneSupprimee = async () => {
  if (!personneCourante.value) return
  mutationEnCours.value = true

  try {
    const reponse = await supprimerPersonne(personneCourante.value.personne_id)
    if (!reponse.success) {
      erreurMutation.value = reponse.error || 'Erreur lors de la suppression'
      return
    }

    // Si la personne supprimée était le centre, reset
    if (centreId.value === personneCourante.value.id) {
      centreId.value = null
    }

    panneauOuvert.value = false
    personneCourante.value = null
    selectedId.value = null

    await rechargerArbre()
  } catch (e: any) {
    erreurMutation.value = e?.data?.error || 'Connexion perdue. Veuillez réessayer.'
  } finally {
    mutationEnCours.value = false
  }
}

// ─── Ajout parent depuis indicateur incomplétude ─────────────────────

const surAjoutParentDemande = (nodeId: string) => {
  const noeud = graphe.value.get(nodeId)
  if (!noeud) return

  selectedId.value = nodeId
  centreId.value = nodeId
  personneCourante.value = noeud
  panneauOuvert.value = true
  surActionAjouter('parent')
}

onMounted(charger)
</script>

<template>
  <div class="relative mt-28 flex h-[calc(100vh-7rem)] flex-col">
    <!-- Barre d'outils -->
    <BarreOutils
      v-if="!estVide && !chargement"
      :mode="mode"
      :nb-branches-incompletes="nbBranchesIncompletes"
      :graphe="graphe"
      @mode-change="surChangementMode"
      @reset-view="surResetVue"
      @recherche-selectionne="surClicNoeud"
    />

    <!-- Zone principale : arbre (gauche) + assistant conversationnel (droite) -->
    <div class="relative flex flex-1 overflow-hidden">
      <!-- Colonne arbre / états -->
      <div class="relative flex flex-1 flex-col">
    <!-- État de chargement -->
    <div v-if="chargement" class="flex flex-1 items-center justify-center">
      <div class="text-center">
        <div class="mx-auto mb-4 h-12 w-12 animate-spin rounded-full border-4 border-stone-200 border-t-[var(--color-custom-chocolat)]" />
        <p class="text-stone-500">Chargement de l'arbre...</p>
      </div>
    </div>

    <!-- Erreur -->
    <div v-else-if="erreur" class="flex flex-1 items-center justify-center">
      <div class="text-center">
        <p class="mb-4 text-lg text-red-600">{{ erreur }}</p>
        <button
          class="rounded-lg bg-[var(--color-custom-chocolat)] px-6 py-2 text-white transition-colors hover:bg-[var(--color-custom-chocolat)]/90"
          @click="reessayer"
        >
          Réessayer
        </button>
      </div>
    </div>

    <!-- État vide -->
    <div v-else-if="estVide" class="flex flex-1 items-center justify-center">
      <div class="text-center">
        <div class="mx-auto mb-4 flex h-20 w-20 items-center justify-center rounded-full bg-stone-100">
          <font-awesome-icon :icon="['fas', 'tree']" class="text-3xl text-stone-400" />
        </div>
        <h2 class="mb-2 text-xl font-semibold text-stone-700">Votre arbre est vide</h2>
        <p class="mb-6 text-stone-500">Laissez-vous guider par l'assistant pour le construire pas à pas.</p>
        <button
          class="inline-flex items-center gap-2 rounded-lg bg-[var(--color-custom-green)] px-6 py-2.5 font-semibold text-white transition-opacity hover:opacity-90"
          @click="assistantOuvert = true"
        >
          <font-awesome-icon :icon="['fas', 'seedling']" />
          Construire mon arbre avec l'assistant
        </button>
      </div>
    </div>

    <!-- Arbre graphique -->
    <div v-else class="relative flex-1">
      <ClientOnly>
        <ArbreGraphe
          ref="arbreGrapheRef"
          :nodes="nodes"
          :edges="edges"
          :selected-id="selectedId"
          @node-click="surClicNoeud"
          @pane-click="surClicFond"
        />
        <template #fallback>
          <div class="flex h-full items-center justify-center">
            <div class="h-12 w-12 animate-spin rounded-full border-4 border-stone-200 border-t-[var(--color-custom-chocolat)]" />
          </div>
        </template>
      </ClientOnly>

      <!-- Wizard ajout contextuel -->
      <AssistantAjoutPersonne
        v-if="afficherWizard"
        :type-lien="wizardTypeLien"
        :type-action="wizardTypeAction"
        :personne-liee="wizardPersonneLiee"
        :loading="mutationEnCours"
        @submit="surWizardSubmit"
        @annuler="surWizardAnnuler"
        @formulaire-classique="surWizardFormulaireClassique"
      />

      <!-- Panneau contextuel -->
      <PanneauPersonne
        v-if="panneauOuvert && personneCourante"
        :personne="personneCourante"
        :mode="modePanneau"
        :contexte-ajout="contexteAjout"
        :mutation-en-cours="mutationEnCours"
        :erreur-mutation="erreurMutation"
        @fermer="fermerPanneau"
        @action-ajouter="surActionAjouter"
        @action-modifier="surActionModifier"
        @action-supprimer="surPersonneSupprimee"
        @personne-ajoutee="surPersonneAjoutee"
        @personne-modifiee="surPersonneModifiee"
        @retour-fiche="surRetourFiche"
      />
      </div>
      <!-- /Arbre graphique -->
      </div>
      <!-- /Colonne arbre -->

      <!-- Colonne assistant conversationnel -->
      <AssistantConversationnel
        v-if="assistantOuvert"
        :cible="cibleAssistant"
        @fermer="assistantOuvert = false"
        @arbre-modifie="rechargerArbre"
      />
    </div>

    <!-- Bouton flottant pour rouvrir l'assistant -->
    <button
      v-if="!assistantOuvert && !chargement && !erreur"
      class="absolute bottom-6 right-6 z-30 flex items-center gap-2 rounded-full bg-[var(--color-custom-green)] px-5 py-3 font-semibold text-white shadow-lg transition-opacity hover:opacity-90"
      @click="assistantOuvert = true"
    >
      <font-awesome-icon :icon="['fas', 'seedling']" />
      Assistant
    </button>
  </div>
</template>
