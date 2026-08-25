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
      <NuxtLink to="/" class="shrink-0">
        <img src="/logos/logo_uafracas.png" alt="AfricanS" class="h-[59px] w-auto" />
      </NuxtLink>

      <!-- Profil. Dans la maquette il n'est pas collé au logo : il commence au
           tiers de la barre, au droit de la colonne principale. -->
      <NuxtLink
        v-if="estConnecte"
        to="/mon-compte/profil"
        class="ml-auto flex items-center gap-3 lg:mr-auto lg:ml-[calc(var(--spacing-af-colonne)+2.5rem)]"
      >
        <img
          v-if="photo"
          :src="photo"
          :alt="nomAffiche"
          class="size-11 rounded-full object-cover"
        />
        <span v-else class="grid size-11 place-items-center rounded-full bg-af-chocolat/15 text-af-chocolat">
          <font-awesome-icon icon="fa-solid fa-user" />
        </span>
        <span class="hidden text-base font-bold sm:inline">{{ nomAffiche }}</span>
      </NuxtLink>

      <div class="ml-auto flex items-center gap-4">
        <template v-if="estConnecte">
          <!-- Cloche : le composant existait déjà, complet (compteur de non-lus,
               liste, marquage lu, navigation vers l'objet) : il n'était monté
               nulle part depuis la refonte. Les trois icônes de la maquette
               étaient des `<button>` sans gestionnaire : décoratives. -->
          <LayoutClocheNotifications />

          <!-- Menu du compte. La barre affiche le nom et la photo, mais le reste
               du menu que portait l'ancienne navigation, engagement, ami(e)s,
               contributions, administration, déconnexion, n'avait plus de
               porte d'entrée. -->
          <div ref="menuRef" class="relative">
            <button
              type="button"
              class="grid size-6 place-items-center text-af-chocolat transition hover:opacity-70 focus-visible:outline-2 focus-visible:outline-offset-4 focus-visible:outline-af-chocolat"
              :aria-expanded="menuOuvert"
              aria-haspopup="true"
              aria-label="Mon compte"
              title="Mon compte"
              @click="menuOuvert = !menuOuvert"
            >
              <font-awesome-icon icon="fa-solid fa-gear" class="text-xl" />
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
  </header>
</template>

<script setup lang="ts">
import { useUserStore } from '~/stores/user'

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

const liensCompte = computed(() => [
  { libelle: 'Mon profil', vers: '/mon-compte/profil', icone: 'fa-solid fa-user' },
  { libelle: 'Mon engagement', vers: '/mon-compte/engagement', icone: 'fa-solid fa-medal' },
  { libelle: 'Mes ami(e)s', vers: '/mon-compte/amis', icone: 'fa-solid fa-user-check' },
  { libelle: 'Mes contributions', vers: '/mon-compte/contributions', icone: 'fa-solid fa-clipboard-list' }, ...(userStore.isAdmin
    ? [{ libelle: 'Administration', vers: '/admin', icone: 'fa-solid fa-shield-halved' }]
    : [])])

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
})
onBeforeUnmount(() => {
  document.removeEventListener('click', surClicExterieur)
  document.removeEventListener('keydown', surEchap)
})
</script>
