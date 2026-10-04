<script setup lang="ts">
/**
 * File des signalements d'épreuve (feature 013).
 *
 * Regroupée PAR ÉPREUVE : dix membres qui signalent la même question sont un
 * seul sujet. Confirmer un signalement confirme d'un coup tous ceux, en
 * attente, de la même épreuve ; chaque signaleur gagne de la réputation.
 *
 * Aucun score déjà gagné sur l'épreuve n'est repris, quelle que soit la
 * décision : l'erreur n'était pas celle du joueur.
 */
import {
  CLASSES_ETAT_EPREUVE,
  LIBELLES_ETAT_EPREUVE,
  LIBELLES_MOTIF_SIGNALEMENT,
  messageErreurAdminJeu,
  type EpreuveSignaleeAPI,
  type SignalementAdminAPI,
} from '~/composables/useAdminJeu'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { listerSignalements, deciderSignalement } = useAdminJeu()

const file = ref<EpreuveSignaleeAPI[]>([])
const etat = ref<SignalementAdminAPI['etat']>('en_attente')
const chargement = ref(true)
const enCours = ref(false)
const erreur = ref('')
const message = ref('')

const ONGLETS: Array<{ valeur: SignalementAdminAPI['etat'], libelle: string }> = [
  { valeur: 'en_attente', libelle: 'En attente' },
  { valeur: 'confirme', libelle: 'Confirmés' },
  { valeur: 'classe', libelle: 'Classés' },
]

const charger = async () => {
  chargement.value = true
  erreur.value = ''
  try {
    file.value = await listerSignalements(etat.value)
  }
  catch (e) {
    erreur.value = messageErreurAdminJeu(e, 'Impossible de charger les signalements.')
  }
  finally {
    chargement.value = false
  }
}

const notifier = (texte: string) => {
  message.value = texte
  setTimeout(() => { if (message.value === texte) message.value = '' }, 5000)
}

const decider = async (
  signalementId: string,
  decision: 'confirmer' | 'classer',
  retirer = false,
) => {
  erreur.value = ''
  enCours.value = true
  try {
    const resultat = await deciderSignalement(signalementId, decision, retirer)
    if (decision === 'classer') notifier('Signalement classé. Le signaleur est prévenu.')
    else {
      const n = resultat?.signalements_traites ?? 1
      notifier(`${n} signalement${n > 1 ? 's' : ''} confirmé${n > 1 ? 's' : ''}`
        + (resultat?.epreuve_retiree ? ', épreuve retirée du jeu.' : '.'))
    }
    await charger()
  }
  catch (e) {
    erreur.value = messageErreurAdminJeu(e)
  }
  finally {
    enCours.value = false
  }
}

const date = (iso: string) => new Date(iso).toLocaleDateString('fr-FR', { day: 'numeric', month: 'short', year: 'numeric' })

watch(etat, charger)
onMounted(charger)
</script>

<template>
  <div>
    <AdminPageHeader
      titre="Signalements d'épreuve"
      sous-titre="Les épreuves que des membres jugent erronées, ambiguës ou déplacées"
    />

    <div v-if="message" class="alert alert-success mb-4 text-sm">{{ message }}</div>
    <div v-if="erreur" class="alert alert-error mb-4 text-sm" role="alert">{{ erreur }}</div>

    <div class="join mb-4">
      <button
        v-for="onglet in ONGLETS"
        :key="onglet.valeur"
        type="button"
        class="btn btn-sm join-item"
        :class="etat === onglet.valeur && 'btn-active'"
        @click="etat = onglet.valeur"
      >
        {{ onglet.libelle }}
      </button>
    </div>

    <div v-if="chargement" class="flex justify-center py-16">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <div v-else-if="file.length === 0" class="card bg-base-100 p-10 text-center text-base-content/60 shadow-sm">
      Aucun signalement ici.
    </div>

    <ul v-else class="space-y-3">
      <li v-for="sujet in file" :key="sujet.epreuve_id" class="card bg-base-100 shadow-sm">
        <div class="card-body gap-3 p-5">
          <div class="flex flex-wrap items-center gap-2 text-xs">
            <span class="badge badge-outline badge-sm">{{ sujet.module_code }}</span>
            <span class="badge badge-sm" :class="CLASSES_ETAT_EPREUVE[sujet.epreuve_etat]">
              {{ LIBELLES_ETAT_EPREUVE[sujet.epreuve_etat] }}
            </span>
            <span class="font-bold text-error">
              {{ sujet.signalements.length }} signalement{{ sujet.signalements.length > 1 ? 's' : '' }}
            </span>
          </div>

          <p class="font-medium">{{ sujet.enonce }}</p>
          <ul class="flex flex-wrap gap-2">
            <li
              v-for="(proposition, index) in sujet.propositions"
              :key="index"
              class="badge"
              :class="index + 1 === sujet.bonne_reponse ? 'badge-success' : 'badge-ghost'"
            >
              {{ proposition }}
            </li>
          </ul>

          <ul class="divide-y divide-base-200 border-t border-base-200">
            <li v-for="s in sujet.signalements" :key="s.id" class="flex flex-wrap items-start gap-3 py-3">
              <div class="min-w-0 flex-1 text-sm">
                <p>
                  <span class="font-bold">{{ LIBELLES_MOTIF_SIGNALEMENT[s.motif] }}</span>
                  <span class="text-base-content/60"> · {{ s.auteur }} · {{ date(s.created_at) }}</span>
                </p>
                <p v-if="s.commentaire" class="mt-1 text-base-content/80">« {{ s.commentaire }} »</p>
              </div>
              <button
                v-if="s.etat === 'en_attente'"
                type="button"
                class="btn btn-ghost btn-xs"
                :disabled="enCours"
                @click="decider(s.id, 'classer')"
              >
                Classer
              </button>
            </li>
          </ul>

          <div v-if="etat === 'en_attente'" class="flex flex-wrap items-center justify-end gap-2 border-t border-base-200 pt-3">
            <NuxtLink :to="`/admin/activites/epreuves/${sujet.epreuve_id}`" class="btn btn-ghost btn-sm mr-auto">
              <font-awesome-icon icon="pen-to-square" class="mr-1" /> Corriger l'épreuve
            </NuxtLink>
            <button
              type="button"
              class="btn btn-outline btn-sm"
              :disabled="enCours"
              @click="decider(sujet.signalements[0]!.id, 'confirmer')"
            >
              Confirmer
            </button>
            <button
              type="button"
              class="btn btn-error btn-sm"
              :disabled="enCours"
              @click="decider(sujet.signalements[0]!.id, 'confirmer', true)"
            >
              Confirmer et retirer l'épreuve
            </button>
          </div>
        </div>
      </li>
    </ul>
  </div>
</template>
