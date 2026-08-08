<script setup lang="ts">
/**
 * Offrir un cadeau virtuel — Tailwind v4 pur (Principe VI, jamais daisyUI côté
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
  <Teleport to="body">
    <Transition name="modal-fade">
      <div
        v-if="isOpen"
        class="fixed inset-0 z-[90] flex items-center justify-center p-4"
        @click.self="fermer"
      >
        <div class="absolute inset-0 bg-black/50 backdrop-blur-sm" />

        <div class="relative w-full max-w-lg overflow-hidden rounded-2xl bg-white shadow-2xl">
          <!-- En-tête -->
          <div class="flex items-center justify-between border-b border-gray-100 px-6 py-4">
            <h3 class="font-oswald text-xl font-bold text-gray-900">
              <font-awesome-icon icon="fa-solid fa-gift" class="mr-2 text-custom-chocolat" />
              Offrir un cadeau
            </h3>
            <button
              type="button"
              class="flex size-9 cursor-pointer items-center justify-center rounded-full text-gray-400 transition-colors hover:bg-gray-100 hover:text-gray-700"
              :disabled="enCours"
              @click="fermer"
            >
              <font-awesome-icon :icon="['fas', 'xmark']" class="size-5" />
            </button>
          </div>

          <div class="max-h-[70vh] overflow-y-auto px-6 py-5">
            <p v-if="chargement" class="py-10 text-center text-sm text-gray-400">
              <font-awesome-icon icon="fa-solid fa-spinner" class="animate-spin" />
              Chargement du catalogue…
            </p>

            <template v-else>
              <p
                v-if="erreur"
                class="mb-4 rounded-xl bg-red-50 px-4 py-2.5 text-sm text-red-700"
              >
                {{ erreur }}
              </p>

              <!-- ─── Étape 1 : choix du cadeau et du mode ─── -->
              <template v-if="etape === 'choix'">
                <EngagementBandeauPaiementSimule
                  v-if="catalogue?.paiement_simule"
                  class="mb-4"
                />

                <p v-if="destinataire" class="mb-3 text-sm text-gray-600">
                  Pour <span class="font-semibold text-gray-900">{{ destinataire }}</span>
                </p>

                <p class="mb-2 text-xs font-medium uppercase tracking-wide text-gray-500">
                  Choisissez un cadeau
                </p>
                <div class="grid grid-cols-2 gap-2.5 sm:grid-cols-3">
                  <button
                    v-for="c in catalogue?.cadeaux ?? []"
                    :key="c.id"
                    type="button"
                    class="flex cursor-pointer flex-col items-center gap-1.5 rounded-xl border-2 px-3 py-3 text-center transition-all hover:shadow-md active:scale-95"
                    :class="cadeauChoisi?.id === c.id
                      ? 'border-custom-green bg-custom-green/5'
                      : 'border-gray-200 bg-white hover:border-gray-300'"
                    @click="cadeauChoisi = c"
                  >
                    <font-awesome-icon
                      :icon="`fa-solid fa-${c.icone || 'gift'}`"
                      class="text-2xl text-custom-chocolat"
                    />
                    <span class="text-xs font-semibold text-gray-800">{{ c.libelle }}</span>
                    <span class="text-[11px] text-gray-500">
                      {{ formaterMontant(c.prix, devise) }}
                    </span>
                    <span class="text-[11px] font-semibold text-custom-green">
                      +{{ c.points }} pt{{ c.points > 1 ? 's' : '' }}
                    </span>
                  </button>
                </div>

                <template v-if="cadeauChoisi">
                  <p class="mb-2 mt-5 text-xs font-medium uppercase tracking-wide text-gray-500">
                    Comment souhaitez-vous l'offrir ?
                  </p>
                  <div class="space-y-2">
                    <label
                      class="flex cursor-pointer items-start gap-3 rounded-xl border-2 p-3 transition-colors"
                      :class="mode === 'soutien_financier'
                        ? 'border-custom-green bg-custom-green/5'
                        : 'border-gray-200 hover:border-gray-300'"
                    >
                      <input v-model="mode" type="radio" value="soutien_financier" class="mt-1 accent-custom-green">
                      <span class="text-sm">
                        <span class="block font-semibold text-gray-900">Soutien financier</span>
                        <span class="block text-xs leading-relaxed text-gray-500">
                          {{ formaterMontant(repartition.beneficiaire, devise) }} vont à la cagnotte
                          du bénéficiaire, {{ formaterMontant(repartition.plateforme, devise) }} à la
                          plateforme. Il reçoit aussi
                          <strong class="text-custom-green">+{{ cadeauChoisi.points }} points</strong>.
                        </span>
                      </span>
                    </label>

                    <label
                      class="flex cursor-pointer items-start gap-3 rounded-xl border-2 p-3 transition-colors"
                      :class="mode === 'points'
                        ? 'border-custom-green bg-custom-green/5'
                        : 'border-gray-200 hover:border-gray-300'"
                    >
                      <input v-model="mode" type="radio" value="points" class="mt-1 accent-custom-green">
                      <span class="text-sm">
                        <span class="block font-semibold text-gray-900">Cadeau en points</span>
                        <span class="block text-xs leading-relaxed text-gray-500">
                          Le bénéficiaire reçoit
                          <strong class="text-custom-green">+{{ cadeauChoisi.points }} points</strong>,
                          sans part financière : la totalité revient à la plateforme.
                        </span>
                      </span>
                    </label>
                  </div>

                  <label class="mt-5 block">
                    <span class="mb-1 block text-xs font-medium text-gray-500">
                      Un mot pour l'accompagner (facultatif)
                    </span>
                    <textarea
                      v-model="message"
                      rows="2"
                      :maxlength="MESSAGE_MAX"
                      class="w-full resize-none rounded-lg border border-gray-300 px-3.5 py-2.5 text-sm focus:border-transparent focus:ring-2 focus:ring-custom-green"
                      placeholder="Merci pour ce contenu !"
                    />
                    <span class="mt-1 block text-right text-[11px] text-gray-400">
                      {{ message.length }} / {{ MESSAGE_MAX }}
                    </span>
                  </label>
                </template>
              </template>

              <!-- ─── Étape 2 : paiement ─── -->
              <template v-else-if="etape === 'paiement' && intention">
                <EngagementBandeauPaiementSimule v-if="intention.simule" class="mb-4" />

                <div class="rounded-2xl border border-gray-100 bg-gray-50 p-4">
                  <dl class="space-y-2 text-sm">
                    <div class="flex justify-between">
                      <dt class="text-gray-500">Cadeau</dt>
                      <dd class="font-semibold text-gray-900">{{ cadeauChoisi?.libelle }}</dd>
                    </div>
                    <div class="flex justify-between">
                      <dt class="text-gray-500">Bénéficiaire</dt>
                      <dd class="font-semibold text-gray-900">
                        {{ intention.beneficiaire.nom_affiche }}
                      </dd>
                    </div>
                    <div class="flex justify-between">
                      <dt class="text-gray-500">Montant</dt>
                      <dd class="font-oswald text-lg font-bold text-custom-chocolat">
                        {{ formaterMontant(intention.montant, devise) }}
                      </dd>
                    </div>
                    <div class="flex justify-between border-t border-gray-200 pt-2 text-xs">
                      <dt class="text-gray-500">Part du bénéficiaire</dt>
                      <dd class="text-gray-700">
                        {{ formaterMontant(intention.part_beneficiaire, devise) }}
                      </dd>
                    </div>
                    <div class="flex justify-between text-xs">
                      <dt class="text-gray-500">Part de la plateforme</dt>
                      <dd class="text-gray-700">
                        {{ formaterMontant(intention.part_plateforme, devise) }}
                      </dd>
                    </div>
                    <div class="flex justify-between border-t border-gray-200 pt-2">
                      <dt class="text-gray-500">Points offerts</dt>
                      <dd class="font-semibold text-custom-green">+{{ intention.points }}</dd>
                    </div>
                  </dl>
                  <p class="mt-3 font-mono text-[11px] text-gray-400">
                    Référence : {{ intention.reference_paiement }}
                  </p>
                </div>

                <p class="mt-4 text-center text-xs text-gray-500">
                  Choisissez l'issue du paiement simulé :
                </p>
              </template>

              <!-- ─── Étape 3a : succès ─── -->
              <div v-else-if="etape === 'succes'" class="py-6 text-center">
                <span class="mx-auto grid size-14 place-items-center rounded-full bg-custom-green/10">
                  <font-awesome-icon icon="fa-solid fa-check" class="text-2xl text-custom-green" />
                </span>
                <p class="mt-4 font-oswald text-lg font-bold text-gray-900">Cadeau envoyé !</p>
                <p class="mt-1 text-sm text-gray-600">
                  {{ intention?.beneficiaire.nom_affiche }} a reçu votre
                  {{ cadeauChoisi?.libelle?.toLowerCase() }}
                  <template v-if="pointsCredites > 0">
                    et <strong class="text-custom-green">+{{ pointsCredites }} points</strong>.
                  </template>
                  <!--
                    Points à zéro = la règle `cadeau_recu` a été désactivée en
                    back-office. Le cadeau reste offert et la répartition
                    journalisée : le dire vaut mieux qu'afficher « +0 points ».
                  -->
                  <template v-else>
                    . L'attribution de points par cadeau est actuellement suspendue.
                  </template>
                </p>
              </div>

              <!-- ─── Étape 3b : échec ─── -->
              <div v-else-if="etape === 'echec'" class="py-6 text-center">
                <span class="mx-auto grid size-14 place-items-center rounded-full bg-red-50">
                  <font-awesome-icon icon="fa-solid fa-xmark" class="text-2xl text-red-500" />
                </span>
                <p class="mt-4 font-oswald text-lg font-bold text-gray-900">Paiement non abouti</p>
                <p class="mt-1 text-sm text-gray-600">
                  Aucun montant n'a été prélevé, aucun point n'a été attribué.
                  Vous pouvez recommencer quand vous le souhaitez.
                </p>
              </div>
            </template>
          </div>

          <!-- Pied -->
          <div class="flex items-center justify-end gap-3 border-t border-gray-100 bg-gray-50 px-6 py-4">
            <template v-if="etape === 'choix'">
              <button
                type="button"
                class="cursor-pointer rounded-lg px-4 py-2 text-sm font-medium text-gray-700 transition-colors hover:bg-gray-200 disabled:opacity-50"
                :disabled="enCours"
                @click="fermer"
              >
                Annuler
              </button>
              <button
                type="button"
                class="inline-flex cursor-pointer items-center gap-2 rounded-lg bg-custom-green px-5 py-2 text-sm font-medium text-white transition-colors hover:bg-custom-green/90 disabled:opacity-60"
                :disabled="!cadeauChoisi || enCours"
                @click="passerAuPaiement"
              >
                <font-awesome-icon v-if="enCours" icon="fa-solid fa-spinner" class="animate-spin" />
                <font-awesome-icon v-else icon="fa-solid fa-arrow-right" />
                Continuer
              </button>
            </template>

            <template v-else-if="etape === 'paiement'">
              <button
                type="button"
                class="cursor-pointer rounded-lg border border-red-200 px-4 py-2 text-sm font-medium text-red-700 transition-colors hover:bg-red-50 disabled:opacity-50"
                :disabled="enCours"
                @click="conclure(false)"
              >
                Simuler un échec
              </button>
              <button
                type="button"
                class="inline-flex cursor-pointer items-center gap-2 rounded-lg bg-custom-green px-5 py-2 text-sm font-medium text-white transition-colors hover:bg-custom-green/90 disabled:opacity-60"
                :disabled="enCours"
                @click="conclure(true)"
              >
                <font-awesome-icon v-if="enCours" icon="fa-solid fa-spinner" class="animate-spin" />
                <font-awesome-icon v-else icon="fa-solid fa-check" />
                Confirmer le paiement
              </button>
            </template>

            <template v-else>
              <button
                type="button"
                class="cursor-pointer rounded-lg bg-custom-green px-5 py-2 text-sm font-medium text-white transition-colors hover:bg-custom-green/90"
                @click="fermer"
              >
                Fermer
              </button>
            </template>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.modal-fade-enter-active,
.modal-fade-leave-active { transition: opacity 0.2s ease; }
.modal-fade-enter-from,
.modal-fade-leave-to { opacity: 0; }
</style>
