<script setup lang="ts">
import type { MembreLightAPI } from '~/composables/useAmis'
import { useUserStore } from '~/stores/user'

/**
 * Messagerie ancrée, sur le modèle de LinkedIn.
 *
 * Trois traits la définissent, et c'est ce qui la sépare de la fenêtre
 * flottante qu'elle remplace :
 *   1. elle est ANCRÉE en bas à droite, pas déplaçable ;
 *   2. repliée, elle reste une BARRE visible et non un bouton rond, la
 *      messagerie annonce sa présence sans occuper l'écran ;
 *   3. la liste se cherche.
 *
 * Ce que LinkedIn n'a pas et que la plateforme garde : les rendez-vous vidéo et
 * l'annuaire des membres. Ils passent en icônes d'en-tête plutôt qu'en onglets,
 * à la place qu'occupent chez LinkedIn le stylo « nouveau message » et le menu.
 */
const { conversations, nonLusTotal, listerConversations, fermerConversation, demandeOuverture } = useMessagerie()
const { nbAttenteMoi, compterAttenteMoi } = useRendezVous()
// Appels directs : sonnerie entrante + salle visio (montés globalement, ci-dessous).
const { appelEntrant, appelActif, accepterAppel, refuserAppel } = useAppels()
const userStore = useUserStore()

// La barre de lecture persistante occupe le bas de l'écran : sans ce décalage,
// le dock passerait dessous et deviendrait inatteignable.
const { aUnContenu: lectureMediaActive } = useLecteurMedia()

const ouvert = ref(false)
const amiSelectionne = ref<MembreLightAPI | null>(null)
const verrouilleeSelection = ref(false)
const chargee = ref(false)
const recherche = ref('')

/** Vue du corps quand aucune conversation n'est ouverte. */
const vue = ref<'discussions' | 'membres' | 'rendezvous'>('discussions')

const badge = computed(() => (nonLusTotal.value > 9 ? '9+' : String(nonLusTotal.value)))
const photoMoi = computed(() => urlMedia(userStore.user?.photo_url))
const nomMoi = computed(() => userStore.fullName || userStore.displayName || 'Mon compte')

const titreEnTete = computed(() => {
  if (amiSelectionne.value) return `${amiSelectionne.value.prenom} ${amiSelectionne.value.nom}`
  if (vue.value === 'membres') return 'Nouveau message'
  if (vue.value === 'rendezvous') return 'Rendez-vous'
  return 'Messagerie'
})

const charger = async () => {
  compterAttenteMoi()
  if (chargee.value) return
  await listerConversations()
  chargee.value = true
}

// Ouverture programmatique depuis l'extérieur (ex. /codi-moi → « Envoyer un message »).
watch(demandeOuverture, async (ami) => {
  if (!ami) return
  ouvert.value = true
  vue.value = 'discussions'
  await charger()
  const conv = conversations.value.find(c => c.ami.id === ami.id)
  amiSelectionne.value = conv ? conv.ami : ami
  verrouilleeSelection.value = conv ? conv.verrouillee : false
  demandeOuverture.value = null
})

const basculer = async () => {
  ouvert.value = !ouvert.value
  if (ouvert.value) await charger()
}

const selectionner = (amiId: string) => {
  const conv = conversations.value.find(c => c.ami.id === amiId)
  if (!conv) return
  amiSelectionne.value = conv.ami
  verrouilleeSelection.value = conv.verrouillee
}

/** Retour à la liste. Remet aussi la vue sur les discussions : revenir d'une
 *  conversation vers l'annuaire n'aurait aucun sens. */
const retourListe = () => {
  amiSelectionne.value = null
  vue.value = 'discussions'
  fermerConversation()
}
</script>

<template>
  <div>
    <!-- Sonnerie d'appel entrant (au-dessus du dock, priorité visuelle) -->
    <SocialAppelEntrantPrompt
      v-if="appelEntrant"
      :appel="appelEntrant"
      @accepter="accepterAppel"
      @refuser="refuserAppel"
    />

    <!-- Salle visio d'un appel direct en cours (plein écran) -->
    <SocialAppelDirectSalle v-if="appelActif" :salle="appelActif" />

    <!-- Dock ancré. Une seule boîte : repliée elle n'est que son en-tête,
         dépliée elle grandit vers le haut. C'est ce qui donne l'impression de
         dépliement plutôt que d'ouverture d'une fenêtre. -->
    <section
      class="fixed right-6 z-50 flex w-[22rem] flex-col overflow-hidden rounded-t-[10px] border border-b-0 border-af-bordure bg-white font-af shadow-2xl transition-[height] duration-200"
      :class="[lectureMediaActive ? 'bottom-24' : 'bottom-0', ouvert ? 'h-[32rem] max-h-[calc(100svh-6rem)]' : 'h-14']"
      aria-label="Messagerie"
    >
      <!-- En-tête : replie et déplie, comme la barre de LinkedIn. -->
      <header class="flex h-14 shrink-0 items-center gap-3 border-b border-af-bordure px-4">
        <button
          v-if="amiSelectionne"
          type="button"
          class="grid size-8 shrink-0 place-items-center rounded-full text-af-corps transition hover:bg-af-fond"
          aria-label="Revenir à la liste"
          @click="retourListe"
        >
          <font-awesome-icon icon="fa-solid fa-chevron-left" />
        </button>

        <button
          v-else
          type="button"
          class="shrink-0"
          :aria-label="ouvert ? 'Replier la messagerie' : 'Déplier la messagerie'"
          @click="basculer"
        >
          <img
            v-if="photoMoi"
            :src="photoMoi"
            :alt="nomMoi"
            class="size-8 rounded-full object-cover"
          />
          <span v-else class="grid size-8 place-items-center rounded-full bg-af-chocolat/15 text-af-chocolat">
            <font-awesome-icon icon="fa-solid fa-user" />
          </span>
        </button>

        <button
          type="button"
          class="min-w-0 flex-1 text-left"
          :aria-expanded="ouvert"
          @click="amiSelectionne ? undefined : basculer()"
        >
          <span class="block truncate text-[16px]/[1.4] font-bold text-af-encre">{{ titreEnTete }}</span>
        </button>

        <!-- Pastille de non-lus : visible surtout replié, c'est là qu'elle sert. -->
        <span
          v-if="nonLusTotal > 0 && !ouvert"
          class="grid size-5 shrink-0 place-items-center rounded-full bg-af-live text-[10px] font-bold text-white"
        >{{ badge }}</span>

        <template v-if="ouvert && !amiSelectionne">
          <button
            type="button"
            class="relative grid size-8 shrink-0 place-items-center rounded-full text-af-corps transition hover:bg-af-fond"
            :class="vue === 'rendezvous' && 'text-af-chocolat'"
            aria-label="Rendez-vous vidéo"
            title="Rendez-vous vidéo"
            @click="vue = vue === 'rendezvous' ? 'discussions' : 'rendezvous'"
          >
            <font-awesome-icon icon="fa-solid fa-video" />
            <span
              v-if="nbAttenteMoi > 0"
              class="absolute -top-0.5 -right-0.5 grid size-4 place-items-center rounded-full bg-af-live text-[9px] font-bold text-white"
            >{{ nbAttenteMoi > 9 ? '9+' : nbAttenteMoi }}</span>
          </button>

          <!-- Équivalent du stylo « nouveau message » de LinkedIn : chez nous,
               écrire à quelqu'un commence par le trouver dans l'annuaire. -->
          <button
            type="button"
            class="grid size-8 shrink-0 place-items-center rounded-full text-af-corps transition hover:bg-af-fond"
            :class="vue === 'membres' && 'text-af-chocolat'"
            aria-label="Nouveau message"
            title="Nouveau message"
            @click="vue = vue === 'membres' ? 'discussions' : 'membres'"
          >
            <font-awesome-icon icon="fa-solid fa-pen-to-square" />
          </button>
        </template>

        <button
          v-if="!amiSelectionne"
          type="button"
          class="grid size-8 shrink-0 place-items-center rounded-full text-af-corps transition hover:bg-af-fond"
          :aria-label="ouvert ? 'Replier' : 'Déplier'"
          @click="basculer"
        >
          <font-awesome-icon
            icon="fa-solid fa-chevron-up"
            class="transition-transform"
            :class="ouvert && 'rotate-180'"
          />
        </button>
      </header>

      <!-- Corps : rendu uniquement déplié, pour que le repli ne garde pas en vie
           une conversation et son flux d'événements. -->
      <template v-if="ouvert">
        <div v-if="amiSelectionne" class="min-h-0 flex-1">
          <SocialFenetreConversation
            :ami="amiSelectionne"
            :verrouillee="verrouilleeSelection"
            @retour="retourListe"
          />
        </div>

        <SocialAnnuaireMembres v-else-if="vue === 'membres'" class="min-h-0 flex-1" />

        <SocialRendezVousListe v-else-if="vue === 'rendezvous'" class="min-h-0 flex-1" />

        <div v-else class="flex min-h-0 flex-1 flex-col">
          <div class="shrink-0 border-b border-af-bordure p-3">
            <label class="relative block">
              <span class="sr-only">Rechercher dans les messages</span>
              <font-awesome-icon
                icon="fa-solid fa-magnifying-glass"
                class="pointer-events-none absolute top-1/2 left-3 -translate-y-1/2 text-af-atone-2"
              />
              <input
                v-model="recherche"
                type="search"
                placeholder="Rechercher dans les messages"
                class="h-9 w-full rounded-lg bg-af-fond pr-3 pl-9 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
              />
            </label>
          </div>

          <div v-if="!conversations.length" class="flex flex-1 flex-col items-center justify-center px-6 text-center">
            <font-awesome-icon icon="fa-solid fa-comments" class="text-3xl text-af-atone-2" />
            <p class="mt-3 text-[14px]/[1.4] font-bold">Aucune conversation</p>
            <p class="mt-1 text-[12px]/[1.4] text-af-corps">
              Faites-vous des ami(e)s pour commencer à discuter.
            </p>
            <AfricansBouton class="mt-4" variante="secondaire" icone="fa-solid fa-users" @click="vue = 'membres'">
              Parcourir les membres
            </AfricansBouton>
          </div>

          <SocialListeAmis v-else class="flex-1" :filtre="recherche" @selectionner="selectionner" />
        </div>
      </template>
    </section>
  </div>
</template>
