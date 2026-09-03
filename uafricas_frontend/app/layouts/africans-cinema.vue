<template>
  <!--
    Gabarit « cinéma » : la vitrine Télé se parcourt comme un catalogue de
    streaming, et la disposition en est la moitié du sujet.

    Trois écarts assumés avec le gabarit `africans`, tous voulus :

      • Fond nuit de bout en bout. La bascule tient dans la classe `af-cinema`,
        qui redéclare les jetons `af-*` (voir `main.css`) : aucun composant
        partagé n'a deux jeux de classes à connaître, et rien ne change hors de
        ce sous-arbre. La vedette plein écran, déjà noire, s'y fond au lieu de
        buter sur un ground clair.

      • Ni navigation latérale, ni rail. Une rangée qui défile veut toute la
        largeur : les 312 px de la colonne de navigation coûtaient une tuile et
        demie par rangée. Le fil d'Ariane garde le retour vers le module, et la
        barre supérieure garde recherche, notifications et compte : rien n'est
        hors d'atteinte, tout est en haut plutôt qu'à gauche : c'est la
        disposition d'un service de VOD.

      • Marges latérales en POURCENTAGE et non conteneur centré à largeur fixe.
        Une piste horizontale se lit d'autant mieux qu'elle frôle les bords ;
        `max-w-af-conteneur` la couperait au milieu d'un grand écran.
  -->
  <div class="af-cinema flex min-h-screen flex-col bg-af-fond font-af text-af-encre antialiased">
    <AfricansBarreSuperieure />

    <slot name="bandeau" />

    <div class="w-full flex-1 px-[4%]">
      <slot name="fil-ariane" />

      <main class="min-w-0 py-8"><slot /></main>
    </div>

    <!-- Services globaux repris du gabarit `africans` : montés hors du <slot/>,
         c'est ce placement qui fait survivre l'écoute et la messagerie à la
         navigation. Les retirer ici casserait les deux. -->
    <ClientOnly>
      <SocialMessagerieFlottante v-if="estConnecte" />
    </ClientOnly>
    <ClientOnly>
      <MediaBarreLecturePersistante />
    </ClientOnly>
  </div>
</template>

<script setup lang="ts">
import { useUserStore } from '~/stores/user'

const userStore = useUserStore()
const estConnecte = computed(() => userStore.isAuthenticated)

/**
 * Le thème est porté par le `body`, en plus du conteneur ci-dessus.
 *
 * Ce n'est pas une ceinture et bretelles : `AfricansModale` (et les autres
 * fenêtres du parcours) se montent par `<Teleport to="body">`, donc HORS du
 * sous-arbre du gabarit. Sans cette ligne, elles ne voient jamais les jetons
 * nuit et s'ouvrent en blanc sur une page noire.
 *
 * `useHead` retire l'attribut au démontage du gabarit : quitter la page Télé
 * rend au reste de l'application son thème clair, sans nettoyage manuel.
 */
useHead({ bodyAttrs: { class: 'af-cinema' } })
</script>
