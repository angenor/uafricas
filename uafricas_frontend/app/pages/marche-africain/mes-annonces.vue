<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <!-- L'image était hotlinkée sur Unsplash ; celle du module existait
           déjà dans le dépôt. -->
      <AfricansBandeauModule
        titre="Mes annonces"
        sous-titre="Gérez vos annonces : modifier, marquer conclue ou supprimer."
        image="/images/marche-afrique.png"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[
          { libelle: 'Opafrica', vers: '/marche-africain' },
          { libelle: 'Afromarket', vers: '/marche-africain' },
          { libelle: 'Mes annonces' },
        ]"
      >
        <template #action>
          <AfricansBouton vers="/marche-africain" icone="fa-solid fa-plus">
            Publier une annonce
          </AfricansBouton>
        </template>
      </AfricansFilAriane>
    </template>

    <div class="flex flex-col gap-6">

      <!-- Chargement -->
      <div v-if="chargement" class="text-center py-16">
        <div class="animate-spin rounded-full h-12 w-12 border-4 border-af-vert border-t-transparent mx-auto mb-4"></div>
        <p class="text-af-atone">Chargement…</p>
      </div>

      <!-- Vide -->
      <div v-else-if="annonces.length === 0" class="text-center py-16 bg-white rounded-2xl shadow-xs">
        <font-awesome-icon :icon="['fas', 'box-open']" class="w-16 h-16 text-af-atone-2 mx-auto mb-4" />
        <h3 class="text-lg font-semibold text-af-corps mb-2">Aucune annonce</h3>
        <p class="text-af-atone">Publiez votre première annonce sur le Marché Africain.</p>
      </div>

      <!-- Liste -->
      <div v-else class="grid gap-5 sm:grid-cols-2">
        <div
          v-for="a in annonces"
          :key="a.id"
          class="bg-white rounded-2xl shadow-sm overflow-hidden flex flex-col"
        >
          <div class="relative aspect-[16/10] bg-af-fond">
            <img v-if="a.photo_url" :src="a.photo_url" :alt="a.titre" class="w-full h-full object-cover" />
  <!-- Pas de `<img src="…placeholder.jpg">` : ce fichier n'a jamais existé, si
       bien qu'une annonce sans photo affichait une image CASSÉE avec son texte
       de remplacement en travers. Un repli qui doit exister sur le disque est un
       repli qui peut manquer ; celui-ci est du balisage, il ne peut pas
       échouer. -->
            <div v-else class="grid h-full w-full place-items-center">
              <font-awesome-icon icon="fa-solid fa-image" class="text-3xl text-af-atone-2" />
            </div>
            <span class="absolute top-3 left-3 px-3 py-1 rounded-full text-xs font-semibold" :class="badgeEtat(a.etat)">
              {{ libelleEtat(a.etat) }}
            </span>
          </div>
          <div class="p-4 flex-1 flex flex-col">
            <p class="text-xs text-af-atone-2 mb-1">{{ a.type_echange }} · {{ a.categorie }}</p>
            <h3 class="font-semibold text-af-encre line-clamp-2 mb-2">{{ a.titre }}</h3>
            <p class="text-af-chocolat font-bold mb-4">{{ formatPrix(a.prix, a.devise) }}</p>

            <div class="mt-auto flex flex-wrap gap-2">
              <NuxtLink
                :to="`/marche-africain/${a.id}`"
                class="px-3 py-1.5 rounded-lg text-sm border border-af-bordure text-af-corps hover:bg-af-fond"
              >
                Voir
              </NuxtLink>
              <button
                class="px-3 py-1.5 rounded-lg text-sm border border-af-bordure text-af-corps hover:bg-af-fond"
                @click="ouvrirEdition(a.id)"
              >
                Modifier
              </button>
              <button
                v-if="a.etat === 'publiee'"
                class="px-3 py-1.5 rounded-lg text-sm border border-af-vert/30 text-af-vert hover:bg-af-vert/5"
                @click="marquerConclue(a.id)"
              >
                Marquer conclue
              </button>
              <button
                class="px-3 py-1.5 rounded-lg text-sm border border-af-live/30 text-af-live hover:bg-af-live/5"
                @click="confirmerSuppression(a.id)"
              >
                Supprimer
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Pagination -->
      <div v-if="totalPages > 1" class="mt-10 flex items-center justify-center gap-2">
        <button
          :disabled="page === 1"
          class="p-2 rounded-lg border border-af-bordure text-af-corps hover:bg-af-fond disabled:opacity-50"
          @click="changerPage(page - 1)"
        >
          <font-awesome-icon :icon="['fas', 'chevron-left']" class="w-4 h-4" />
        </button>
        <span class="px-4 py-2 text-sm text-af-corps">Page {{ page }} / {{ totalPages }}</span>
        <button
          :disabled="page === totalPages"
          class="p-2 rounded-lg border border-af-bordure text-af-corps hover:bg-af-fond disabled:opacity-50"
          @click="changerPage(page + 1)"
        >
          <font-awesome-icon :icon="['fas', 'chevron-right']" class="w-4 h-4" />
        </button>
      </div>
    </div>

    <!-- Modal édition -->
    <!-- Coque partagée : le Teleport maison redoublait ce qu'AfricansModale
         porte déjà (fermeture au clavier, piège de focus, blocage du
         défilement de fond), et n'en avait aucun. -->
    <AfricansModale
      :model-value="showEdition"
      titre="Modifier l'annonce"
      sous-titre="Vente, troc, don ou opportunité d'investissement"
      icone="fa-solid fa-pen"
      taille="large"
      @update:model-value="showEdition = false"
    >
      <MarcheAnnonceForm
        v-if="annonceEnEdition"
        mode="edition"
        :annonce="annonceEnEdition"
        @success="onEditionReussie"
        @cancel="showEdition = false"
      />
    </AfricansModale>
  </NuxtLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import {
  useMarcheAfricain,
  formatPrix,
  type MesAnnonceItemAPI,
  type AnnonceDetailAPI,
} from '~/composables/useMarcheAfricain'

definePageMeta({ middleware: 'auth', layout: false })

useHead({ title: 'Mes annonces - Marché Africain - AfricanS' })

const {
  chargement,
  mesAnnonces,
  obtenirAnnonce,
  conclureAnnonce,
  supprimerAnnonce,
} = useMarcheAfricain()

const annonces = ref<MesAnnonceItemAPI[]>([])
const page = ref(1)
const totalPages = ref(1)
const showEdition = ref(false)
const annonceEnEdition = ref<AnnonceDetailAPI | null>(null)

const PAR_PAGE = 12

const charger = async () => {
  const r = await mesAnnonces({ page: page.value, par_page: PAR_PAGE })
  if (r) {
    annonces.value = r.annonces
    totalPages.value = r.total_pages
  }
}

const changerPage = (p: number) => {
  if (p >= 1 && p <= totalPages.value) {
    page.value = p
    charger()
    window.scrollTo({ top: 0, behavior: 'smooth' })
  }
}

const libelleEtat = (etat: string): string => {
  const map: Record<string, string> = {
    publiee: 'Publiée',
    conclue: 'Conclue',
    suspendue: 'Suspendue',
    expiree: 'Expirée',
    brouillon: 'Brouillon',
    en_attente: 'En attente',
    supprimee: 'Supprimée',
  }
  return map[etat] || etat
}

const badgeEtat = (etat: string): string => {
  switch (etat) {
    case 'publiee': return 'bg-af-vert/10 text-af-vert'
    case 'conclue': return 'bg-af-bordure text-af-corps'
    case 'suspendue': return 'bg-af-live/10 text-af-live'
    default: return 'bg-af-chocolat/10 text-af-chocolat'
  }
}

const ouvrirEdition = async (id: string) => {
  const detail = await obtenirAnnonce(id)
  if (detail) {
    annonceEnEdition.value = detail
    showEdition.value = true
  }
}

const onEditionReussie = async () => {
  showEdition.value = false
  annonceEnEdition.value = null
  await charger()
}

const marquerConclue = async (id: string) => {
  if (!confirm('Marquer cette annonce comme conclue ? Elle ne sera plus visible publiquement.')) return
  const ok = await conclureAnnonce(id)
  if (ok) await charger()
}

const confirmerSuppression = async (id: string) => {
  if (!confirm('Supprimer définitivement cette annonce ?')) return
  const ok = await supprimerAnnonce(id)
  if (ok) await charger()
}

onMounted(charger)
</script>
