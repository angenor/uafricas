<script setup lang="ts">
/**
 * Offrir un cadeau virtuel : Tailwind v4 pur (Principe VI, jamais daisyUI côté
 * membre). Reprend la coque des modales de partage de la plateforme.
 *
 * Trois étapes, dans cet ordre : choisir le cadeau et le mode → payer → issue.
 * Le parcours de paiement propose **explicitement** l'aboutissement ET l'échec :
 * un simulateur qui ne saurait qu'aboutir rendrait invérifiable la garantie
 * « échec = 0 point, 0 répartition ».
 */
import { computed, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import {
  useCadeaux, formaterMontant,
  type Cadeau, type Catalogue, type IntentionPaiement, type ModeCadeau,
} from '~/composables/useCadeaux'
import { useUserStore } from '~/stores/user'

const props = defineProps<{
  isOpen: boolean
  /** Famille du contenu, ou `'profil'` pour un cadeau offert depuis un profil. */
  typeObjet: string
  /** Identifiant du contenu, ou du membre si `typeObjet === 'profil'`. */
  objetId: string
  /** Nom du destinataire, affiché à titre indicatif avant résolution serveur. */
  destinataire?: string
}>()

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'offert', points: number): void
}>()

const { obtenirCatalogue, envoyerCadeau, confirmerPaiement } = useCadeaux()
const userStore = useUserStore()

type Etape = 'choix' | 'paiement' | 'succes' | 'echec'

const etape = ref<Etape>('choix')
const catalogue = ref<Catalogue | null>(null)
const chargement = ref(true)
const enCours = ref(false)
const erreur = ref('')

const cadeauChoisi = ref<Cadeau | null>(null)
const mode = ref<ModeCadeau>('soutien_financier')
const message = ref('')
const intention = ref<IntentionPaiement | null>(null)
const pointsCredites = ref(0)

const MESSAGE_MAX = 280

/**
 * Répartition annoncée AVANT le paiement, calculée exactement comme le serveur
 * (division entière, part plateforme par différence). Annoncer 1 800 puis en
 * journaliser 1 799 détruirait la confiance dans tout le reste de l'écran.
 */
const repartition = computed(() => {
  const c = cadeauChoisi.value
  const taux = catalogue.value?.taux_commission ?? 10
  if (!c) return { beneficiaire: 0, plateforme: 0 }
  const beneficiaire = mode.value === 'points' ? 0 : Math.floor((c.prix * (100 - taux)) / 100)
  return { beneficiaire, plateforme: c.prix - beneficiaire }
})

const devise = computed(() => catalogue.value?.devise ?? 'XOF')

const reinitialiser = () => {
  etape.value = 'choix'
  cadeauChoisi.value = null
  mode.value = 'soutien_financier'
  message.value = ''
  intention.value = null
  pointsCredites.value = 0
  erreur.value = ''
  enCours.value = false
}

const charger = async () => {
  chargement.value = true
  erreur.value = ''
  try {
    catalogue.value = await obtenirCatalogue()
  } catch {
    erreur.value = 'Le catalogue des cadeaux est momentanément indisponible.'
  } finally {
    chargement.value = false
  }
}

watch(() => props.isOpen, (ouvert) => {
  if (ouvert) {
    reinitialiser()
    charger()
  }
})

const fermer = () => {
  if (enCours.value) return
  emit('close')
}

/** Message d'erreur serveur affiché tel quel : il est déjà rédigé en français. */
const signaler = (e: unknown, defaut: string) => {
  const data = (e as { data?: { error?: string } })?.data
  erreur.value = data?.error || defaut
}

const passerAuPaiement = async () => {
  if (!cadeauChoisi.value) return
  if (!userStore.accessToken) {
    erreur.value = 'Connectez-vous pour offrir un cadeau.'
    return
  }

  enCours.value = true
  erreur.value = ''
  try {
    intention.value = await envoyerCadeau(
      cadeauChoisi.value.id,
      mode.value,
      props.typeObjet,
      props.objetId,
      message.value,
    )
    if (intention.value) etape.value = 'paiement'
  } catch (e) {
    signaler(e, "L'envoi n'a pas pu être préparé.")
  } finally {
    enCours.value = false
  }
}

/**
 * Issue du paiement simulé. `aboutir = false` est un chemin de premier ordre,
 * pas une trappe de test : il doit être aussi accessible que la réussite.
 */
const conclure = async (aboutir: boolean) => {
  if (!intention.value) return
  enCours.value = true
  erreur.value = ''
  try {
    const res = await confirmerPaiement(intention.value.reference_paiement, aboutir)
    if (res?.etat === 'abouti') {
      pointsCredites.value = res.points_credites
      etape.value = 'succes'
      emit('offert', res.points_credites)
    } else {
      etape.value = 'echec'
    }
  } catch (e) {
    signaler(e, "La confirmation du paiement a échoué.")
    etape.value = 'echec'
  } finally {
    enCours.value = false
  }
}

const onKeydown = (e: KeyboardEvent) => {
  if (e.key === 'Escape' && props.isOpen) fermer()
}
onMounted(() => window.addEventListener('keydown', onKeydown))
onBeforeUnmount(() => window.removeEventListener('keydown', onKeydown))
</script>

<template>
  <AfricansModale
    :model-value="isOpen"
    titre="Offrir un cadeau"
    :sous-titre="destinataire ? `Pour ${destinataire}` : undefined"
    icone="fa-solid fa-gift"
    @update:model-value="fermer()"
  >
    <p v-if="chargement" class="py-10 text-center text-[14px]/[1.4] text-af-atone">
      <font-awesome-icon icon="fa-solid fa-spinner" class="animate-spin" />
      Chargement du catalogue…
    </p>

    <template v-else>
      <p
        v-if="erreur"
        class="mb-4 rounded-lg border border-af-live/20 bg-af-live/5 px-4 py-3 text-[14px]/[1.4] text-af-live"
      >
        {{ erreur }}
      </p>

      <!-- ─── Étape 1 : choix du cadeau et du mode ─── -->
      <template v-if="etape === 'choix'">
        <EngagementBandeauPaiementSimule v-if="catalogue?.paiement_simule" class="mb-4" />

        <p class="mb-2 text-[14px]/[1.4] text-af-atone italic">Choisissez un cadeau</p>
        <div class="grid grid-cols-2 gap-2.5 sm:grid-cols-3">
          <button
            v-for="c in catalogue?.cadeaux ?? []"
            :key="c.id"
            type="button"
            class="flex flex-col items-center gap-1.5 rounded-lg border-2 px-3 py-3 text-center transition active:scale-95"
            :class="cadeauChoisi?.id === c.id
              ? 'border-af-vert bg-af-vert/5'
              : 'border-af-bordure bg-white hover:border-af-chocolat'"
            @click="cadeauChoisi = c"
          >
            <font-awesome-icon :icon="`fa-solid fa-${c.icone || 'gift'}`" class="text-2xl text-af-chocolat" />
            <span class="text-[12px] font-bold text-af-encre">{{ c.libelle }}</span>
            <span class="text-[11px] text-af-atone">{{ formaterMontant(c.prix, devise) }}</span>
            <span class="text-[11px] font-bold text-af-vert">
              +{{ c.points }} pt{{ c.points > 1 ? 's' : '' }}
            </span>
          </button>
        </div>

        <template v-if="cadeauChoisi">
          <p class="mt-5 mb-2 text-[14px]/[1.4] text-af-atone italic">
            Comment souhaitez-vous l'offrir ?
          </p>
          <div class="flex flex-col gap-2">
            <label
              class="flex cursor-pointer items-start gap-3 rounded-lg border-2 p-3 transition"
              :class="mode === 'soutien_financier'
                ? 'border-af-vert bg-af-vert/5'
                : 'border-af-bordure hover:border-af-chocolat'"
            >
              <input v-model="mode" type="radio" value="soutien_financier" class="mt-1 accent-af-vert">
              <span>
                <span class="block text-[14px]/[1.4] font-bold text-af-encre">Soutien financier</span>
                <span class="mt-0.5 block text-[12px]/[1.6] text-af-corps">
                  {{ formaterMontant(repartition.beneficiaire, devise) }} vont à la cagnotte
                  du bénéficiaire, {{ formaterMontant(repartition.plateforme, devise) }} à la
                  plateforme. Il reçoit aussi
                  <strong class="font-bold text-af-vert">+{{ cadeauChoisi.points }} points</strong>.
                </span>
              </span>
            </label>

            <label
              class="flex cursor-pointer items-start gap-3 rounded-lg border-2 p-3 transition"
              :class="mode === 'points'
                ? 'border-af-vert bg-af-vert/5'
                : 'border-af-bordure hover:border-af-chocolat'"
            >
              <input v-model="mode" type="radio" value="points" class="mt-1 accent-af-vert">
              <span>
                <span class="block text-[14px]/[1.4] font-bold text-af-encre">Cadeau en points</span>
                <span class="mt-0.5 block text-[12px]/[1.6] text-af-corps">
                  Le bénéficiaire reçoit
                  <strong class="font-bold text-af-vert">+{{ cadeauChoisi.points }} points</strong>,
                  sans part financière : la totalité revient à la plateforme.
                </span>
              </span>
            </label>
          </div>

          <div class="mt-5">
            <AfricansChamp
              v-model="message"
              libelle="Un mot pour l'accompagner"
              type="textarea"
              :lignes="2"
              :maxlength="MESSAGE_MAX"
              placeholder="Merci pour ce contenu !"
              aide="Facultatif"
            />
            <p class="mt-1 text-right text-[11px] text-af-atone-2">
              {{ message.length }} / {{ MESSAGE_MAX }}
            </p>
          </div>
        </template>
      </template>

      <!-- ─── Étape 2 : paiement ─── -->
      <template v-else-if="etape === 'paiement' && intention">
        <EngagementBandeauPaiementSimule v-if="intention.simule" class="mb-4" />

        <div class="rounded-lg border border-af-bordure bg-af-fond p-4">
          <dl class="flex flex-col gap-2 text-[14px]/[1.4]">
            <div class="flex justify-between gap-4">
              <dt class="text-af-atone">Cadeau</dt>
              <dd class="font-bold text-af-encre">{{ cadeauChoisi?.libelle }}</dd>
            </div>
            <div class="flex justify-between gap-4">
              <dt class="text-af-atone">Bénéficiaire</dt>
              <dd class="font-bold text-af-encre">{{ intention.beneficiaire.nom_affiche }}</dd>
            </div>
            <div class="flex justify-between gap-4">
              <dt class="text-af-atone">Montant</dt>
              <dd class="text-[18px] font-bold text-af-chocolat">
                {{ formaterMontant(intention.montant, devise) }}
              </dd>
            </div>
            <div class="flex justify-between gap-4 border-t border-af-bordure pt-2 text-[12px]">
              <dt class="text-af-atone">Part du bénéficiaire</dt>
              <dd class="text-af-corps">{{ formaterMontant(intention.part_beneficiaire, devise) }}</dd>
            </div>
            <div class="flex justify-between gap-4 text-[12px]">
              <dt class="text-af-atone">Part de la plateforme</dt>
              <dd class="text-af-corps">{{ formaterMontant(intention.part_plateforme, devise) }}</dd>
            </div>
            <div class="flex justify-between gap-4 border-t border-af-bordure pt-2">
              <dt class="text-af-atone">Points offerts</dt>
              <dd class="font-bold text-af-vert">+{{ intention.points }}</dd>
            </div>
          </dl>
          <p class="mt-3 font-mono text-[11px] text-af-atone-2">
            Référence : {{ intention.reference_paiement }}
          </p>
        </div>

        <p class="mt-4 text-center text-[12px]/[1.4] text-af-atone">
          Choisissez l'issue du paiement simulé :
        </p>
      </template>

      <!-- ─── Étape 3a : succès ─── -->
      <div v-else-if="etape === 'succes'" class="flex flex-col items-center gap-3 py-6 text-center">
        <span class="grid size-14 place-items-center rounded-full bg-af-vert/10">
          <font-awesome-icon icon="fa-solid fa-check" class="text-2xl text-af-vert" />
        </span>
        <p class="text-[18px]/[1.4] font-bold text-af-encre">Cadeau envoyé !</p>
        <p class="text-[14px]/[1.6] text-af-corps">
          {{ intention?.beneficiaire.nom_affiche }} a reçu votre
          {{ cadeauChoisi?.libelle?.toLowerCase() }}
          <template v-if="pointsCredites > 0">
            et <strong class="font-bold text-af-vert">+{{ pointsCredites }} points</strong>.
          </template>
          <!--
            Points à zéro = la règle `cadeau_recu` a été désactivée en
            back-office. Le cadeau reste offert et la répartition journalisée :
            le dire vaut mieux qu'afficher « +0 points ».
          -->
          <template v-else>
            . L'attribution de points par cadeau est actuellement suspendue.
          </template>
        </p>
      </div>

      <!-- ─── Étape 3b : échec ─── -->
      <div v-else-if="etape === 'echec'" class="flex flex-col items-center gap-3 py-6 text-center">
        <span class="grid size-14 place-items-center rounded-full bg-af-live/10">
          <font-awesome-icon icon="fa-solid fa-xmark" class="text-2xl text-af-live" />
        </span>
        <p class="text-[18px]/[1.4] font-bold text-af-encre">Paiement non abouti</p>
        <p class="text-[14px]/[1.6] text-af-corps">
          Aucun montant n'a été prélevé, aucun point n'a été attribué.
          Vous pouvez recommencer quand vous le souhaitez.
        </p>
      </div>
    </template>

    <template #actions>
      <template v-if="etape === 'choix'">
        <button
          type="button"
          class="text-base font-bold text-af-corps transition hover:opacity-70 disabled:opacity-50"
          :disabled="enCours"
          @click="fermer"
        >
          Annuler
        </button>
        <AfricansBouton
          :desactive="!cadeauChoisi || enCours"
          :tourne="enCours"
          :icone="enCours ? 'fa-solid fa-spinner' : 'fa-solid fa-arrow-right'"
          @click="passerAuPaiement"
        >
          Continuer
        </AfricansBouton>
      </template>

      <template v-else-if="etape === 'paiement'">
        <!-- L'échec est offert EXPLICITEMENT : sans lui, « échec = 0 point »
             resterait invérifiable pendant la phase de test. -->
        <button
          type="button"
          class="rounded-lg border border-af-live/40 px-5 py-2 text-base font-bold text-af-live transition hover:bg-af-live/5 disabled:opacity-50"
          :disabled="enCours"
          @click="conclure(false)"
        >
          Simuler un échec
        </button>
        <AfricansBouton
          :desactive="enCours"
          :tourne="enCours"
          :icone="enCours ? 'fa-solid fa-spinner' : 'fa-solid fa-check'"
          @click="conclure(true)"
        >
          Confirmer le paiement
        </AfricansBouton>
      </template>

      <template v-else>
        <AfricansBouton @click="fermer">Fermer</AfricansBouton>
      </template>
    </template>
  </AfricansModale>
</template>
