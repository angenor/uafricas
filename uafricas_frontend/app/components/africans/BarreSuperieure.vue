<template>
  <!-- Barre supérieure de la maquette : 89 px, blanche. Le logo est aligné sur
       le bord GAUCHE du conteneur (x=174 dans Figma), pas sur celui du viewport.
       ÉCART ASSUMÉ : elle était non collante au lot 1, comme le dessin. Le fil
       d'actualité est devenu la racine et défile sans fin ; le profil, les
       notifications et la recherche partaient hors d'atteinte. Elle est donc
       `sticky`, avec un filet qui la détache du contenu qui passe dessous.
       `z-50` et non davantage : les modales montent à `z-100` et doivent
       continuer de la recouvrir. -->
  <header class="sticky top-0 z-50 h-af-barre border-b border-af-bordure bg-white">
    <div class="mx-auto flex h-full max-w-af-conteneur items-center gap-6 px-6">
      <!-- Trois zones, dont DEUX ÉLASTIQUES de part et d'autre. C'est ce qui
           centre réellement la recherche : un simple `mx-auto` la centrerait
           dans l'espace RESTANT, et se décalerait donc dès que le logo et les
           contrôles de droite n'ont pas la même largeur, ce qui est le cas ici.
           `min-w-0` les autorise à se comprimer plutôt qu'à déborder. -->
      <div class="flex min-w-0 flex-1 items-center">
        <NuxtLink to="/" class="shrink-0">
          <img src="/logos/logo_uafracas.png" alt="AfricanS" class="h-[59px] w-auto" />
        </NuxtLink>
      </div>

      <!-- Recherche globale. C'est un BOUTON déguisé en champ, pas un champ :
           la saisie et les résultats vivent dans la fenêtre de recherche, qui
           les groupe par nature et se pilote au clavier. Deux champs, l'un dans
           la barre et l'autre dans la fenêtre qu'il ouvre, obligeraient à
           retaper ce qu'on vient d'écrire.

           Largeur FIXE, et `shrink-0` : c'est l'élément du milieu, et un
           élément central qui se comprime n'est plus centré sur la même chose
           d'un écran à l'autre. 320 px suffisent à sa phrase.

           Seuil à 48rem : à 640 px, logo + 320 px de champ + les deux
           contrôles de droite débordent. En dessous, la loupe remplace le
           champ.

           Elle est posée entre les deux zones élastiques, pas poussée par
           une marge : c'est la seule façon qu'elle reste au milieu quand le
           nom affiché à droite s'allonge. -->
      <button
        type="button"
        class="hidden h-11 w-80 shrink-0 items-center gap-3 rounded-lg border border-af-bordure bg-af-fond px-4 text-left transition hover:border-af-chocolat focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-af-chocolat md:flex"
        aria-label="Rechercher sur AfricanS"
        @click="rechercheOuverte = true"
      >
        <font-awesome-icon icon="fa-solid fa-magnifying-glass" class="shrink-0 text-af-atone-2" />
        <span class="min-w-0 flex-1 truncate text-[14px]/[1.4] text-af-atone-2">Rechercher sur AfricanS…</span>
        <kbd class="hidden shrink-0 rounded border border-af-bordure bg-white px-1.5 py-0.5 text-[11px] text-af-atone lg:inline">
          {{ raccourci }}
        </kbd>
      </button>

      <div class="flex min-w-0 flex-1 items-center justify-end gap-4">
        <!-- Repli sous 48rem, où le champ disparaît. Hors de la branche
             « connecté » comme le champ lui-même : la racine sert le fil à
             tout le monde, un visiteur doit pouvoir chercher aussi. -->
        <button
          type="button"
          class="grid size-6 shrink-0 place-items-center text-af-chocolat transition hover:opacity-70 md:hidden"
          aria-label="Rechercher sur AfricanS"
          @click="rechercheOuverte = true"
        >
          <font-awesome-icon icon="fa-solid fa-magnifying-glass" class="text-xl" />
        </button>

        <template v-if="estConnecte">
          <!-- Cloche : le composant existait déjà, complet (compteur de non-lus,
               liste, marquage lu, navigation vers l'objet) : il n'était monté
               nulle part depuis la refonte. Les trois icônes de la maquette
               étaient des `<button>` sans gestionnaire : décoratives. -->
          <LayoutClocheNotifications />

          <!-- Menu du compte : profil, engagement, ami(e)s, contributions,
               administration, déconnexion. Ces entrées que portait l'ancienne
               navigation n'avaient plus de porte d'entrée depuis la refonte. -->
          <div ref="menuRef" class="relative">
            <!-- Avatar ET menu en un seul contrôle. Ils faisaient double
                 emploi : l'avatar menait au profil, l'engrenage ouvrait un
                 menu dont la PREMIÈRE entrée est « Mon profil ». Deux points
                 d'entrée pour la même destination, à deux endroits différents
                 de la barre.

                 Le profil reste donc atteignable, en une entrée nommée plutôt
                 qu'en devinant qu'une photo est cliquable. -->
            <button
              type="button"
              class="flex items-center gap-2 rounded-full pr-2 transition hover:bg-af-fond focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-af-chocolat"
              :aria-expanded="menuOuvert"
              aria-haspopup="true"
              aria-label="Mon compte"
              @click="menuOuvert = !menuOuvert"
            >
              <img
                v-if="photo"
                :src="photo"
                :alt="''"
                class="size-11 shrink-0 rounded-full object-cover"
              />
              <span v-else class="grid size-11 shrink-0 place-items-center rounded-full bg-af-chocolat/15 text-af-chocolat">
                <font-awesome-icon icon="fa-solid fa-user" />
              </span>
              <span class="hidden max-w-40 truncate text-base font-bold lg:inline">{{ nomAffiche }}</span>
              <font-awesome-icon
                icon="fa-solid fa-chevron-down"
                class="shrink-0 text-sm text-af-corps transition-transform"
                :class="menuOuvert && 'rotate-180'"
              />
            </button>

            <div
              v-if="menuOuvert"
              class="absolute right-0 z-50 mt-3 w-60 overflow-hidden rounded-[10px] border border-af-bordure bg-white py-2 shadow-lg"
            >
              <NuxtLink
                v-for="lien in liensCompte"
                :key="lien.vers"
                :to="lien.vers"
                class="flex items-center gap-3 px-4 py-2.5 text-[14px]/[1.4] text-af-corps transition hover:bg-af-fond"
                @click="menuOuvert = false"
              >
                <font-awesome-icon :icon="lien.icone" class="w-4 shrink-0 text-af-atone" />
                {{ lien.libelle }}
              </NuxtLink>

              <button
                type="button"
                class="flex w-full items-center gap-3 border-t border-af-bordure px-4 py-2.5 text-left text-[14px]/[1.4] text-af-corps transition hover:bg-af-fond hover:text-af-live"
                @click="seDeconnecter"
              >
                <font-awesome-icon icon="fa-solid fa-right-from-bracket" class="w-4 shrink-0 text-af-atone" />
                Se déconnecter
              </button>
            </div>
          </div>

          <!-- Il n'existe AUCUNE page d'aide dans le projet : ce lien mène à la
               présentation de la plateforme, la seule page qui explique ce
               qu'on peut y faire. Un vrai centre d'aide reste à écrire. -->
          <NuxtLink
            to="/decouvrir"
            class="grid size-6 place-items-center text-af-chocolat transition hover:opacity-70 focus-visible:outline-2 focus-visible:outline-offset-4 focus-visible:outline-af-chocolat"
            aria-label="Découvrir AfricanS"
            title="Découvrir AfricanS"
          >
            <font-awesome-icon icon="fa-solid fa-circle-question" class="text-xl" />
          </NuxtLink>
        </template>

        <template v-else>
          <!-- La racine sert le fil d'actualité à TOUT LE MONDE : un visiteur y
               arrive donc sans savoir ce qu'est AfricanS. Ce lien est sa seule
               sortie vers la page qui le lui dit. -->
          <NuxtLink
            to="/decouvrir"
            class="flex items-center gap-2 text-base font-bold text-af-encre transition hover:text-af-chocolat"
          >
            <font-awesome-icon icon="fa-solid fa-earth-africa" />
            <span class="hidden sm:inline">Découvrir AfricanS</span>
          </NuxtLink>

          <NuxtLink
            to="/login"
            class="rounded-lg bg-af-degrade px-6 py-2.5 text-base font-bold text-white transition hover:opacity-90"
          >
            Se Connecter
          </NuxtLink>
        </template>
      </div>
    </div>

    <LayoutRecherchePopup :ouvert="rechercheOuverte" @fermer="rechercheOuverte = false" />
  </header>
</template>

<script setup lang="ts">
import { useUserStore } from '~/stores/user'
import { NAV_COMPTE } from '~/utils/navigation-compte'

const userStore = useUserStore()
const { logout } = useAuth()

const estConnecte = computed(() => userStore.isAuthenticated)
// La maquette affiche le nom complet (« Wade Warren »), pas le seul prénom.
const nomAffiche = computed(() => userStore.fullName || userStore.displayName || 'Mon compte')
// `urlMedia` et non le chemin brut : le backend renvoie du relatif, servi
// sur SON port. Voir `utils/media.ts`.
const photo = computed(() => urlMedia(userStore.user?.photo_url))

const menuOuvert = ref(false)
const menuRef = ref<HTMLElement | null>(null)

// ── Recherche globale ─────────────────────────────────────────────────────
// Le composant `LayoutRecherchePopup` était complet (résultats groupés,
// navigation au clavier, recherches récentes) mais n'était monté que par
// l'ancienne barre : les pages du gabarit n'avaient aucune recherche.
const rechercheOuverte = ref(false)

// Affiché tel quel dans la pastille : `⌘K` sur macOS, `Ctrl K` ailleurs.
// Le calcul est CLIENT uniquement — `navigator` n'existe pas au rendu serveur.
const raccourci = ref('Ctrl K')

const surRaccourciRecherche = (e: KeyboardEvent) => {
  if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === 'k') {
    e.preventDefault()
    rechercheOuverte.value = true
  }
}

// Le menu déroulant offre un RACCOURCI, pas l'espace entier : huit entrées
// surplombant chaque page ne seraient plus un raccourci. Le rail des pages de
// compte, lui, les donne toutes. Une seule source les décrit.
const liensCompte = computed(() => [
  ...NAV_COMPTE.filter(e => e.dansLeMenu),
  ...(userStore.isAdmin
    ? [{ libelle: 'Administration', vers: '/admin', icone: 'fa-solid fa-shield-halved' }]
    : []),
])

const seDeconnecter = async () => {
  menuOuvert.value = false
  await logout()
}

// Fermeture au clic extérieur ET à Échap : un menu qu'on ne peut refermer que
// par le bouton qui l'a ouvert piège l'utilisateur.
const surClicExterieur = (e: MouseEvent) => {
  if (menuOuvert.value && menuRef.value && !menuRef.value.contains(e.target as Node)) {
    menuOuvert.value = false
  }
}
const surEchap = (e: KeyboardEvent) => {
  if (e.key === 'Escape') menuOuvert.value = false
}

onMounted(() => {
  document.addEventListener('click', surClicExterieur)
  document.addEventListener('keydown', surEchap)
  document.addEventListener('keydown', surRaccourciRecherche)
  if (/Mac|iPhone|iPad/.test(navigator.platform || navigator.userAgent)) {
    raccourci.value = '⌘K'
  }
})
onBeforeUnmount(() => {
  document.removeEventListener('click', surClicExterieur)
  document.removeEventListener('keydown', surEchap)
  document.removeEventListener('keydown', surRaccourciRecherche)
})
</script>
