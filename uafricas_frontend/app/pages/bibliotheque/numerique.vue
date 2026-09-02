<script setup lang="ts">
import { documentTypes as typesBase } from '~/mocks/bibliotheques'
import type { LivreAPI } from '~/composables/useBibliotheque'

/**
 * Librafrica : porté sur le gabarit de la refonte.
 *
 * Le filtrage reste côté serveur (recherche débattue à 300 ms + type de
 * document), la lecture passe toujours par `CommonPdfViewer`, et le dépôt d'un
 * document garde ses neuf champs : extraits dans `ProposerDocumentModale`,
 * qui les tenait en 200 lignes au milieu de la grille.
 *
 * Deux corrections au passage :
 *   - la couverture de repli pointait sur **images.unsplash.com** ; un document
 *     sans image partait chercher un JPEG chez un tiers. Elle devient une
 *     vignette locale (icône sur le fond de la charte) ;
 *   - les deux refus bloquants du dépôt (PDF manquant, consentement non coché)
 *     passaient par `alert()`. Ils s'affichent dans la modale, à côté du champ.
 */
definePageMeta({ layout: false })

useHead({
  title: 'Librafrica : Bibliothèque numérique | AfricanS',
  meta: [
    { name: 'description', content: "Accédez à des milliers de livres et documents africains." }],
})

const { chargement, erreur, listerLivres, creerLivre } = useBibliotheque()

// ─── État ─────────────────────────────────────────────────────────────────

const TYPES_DOCUMENT = ['Tous', ...typesBase]

const livres = ref<LivreAPI[]>([])
const searchQuery = ref('')
const selectedType = ref('Tous')
const decouverteOuverte = ref(false)

const pdfSelected = ref<{ url: string | null, acces: string | null }>({ url: null, acces: null })

const modaleDepot = ref(false)
const isSubmitting = ref(false)
const erreurSoumission = ref<string | null>(null)

// ─── Chargement (filtrage côté serveur) ───────────────────────────────────

async function chargerLivres() {
  const filtres: Record<string, any> = { par_page: 20 }

  if (searchQuery.value.trim()) filtres.recherche = searchQuery.value.trim()

  // « Autre » n'est pas un type stocké : c'est le fourre-tout du formulaire.
  if (selectedType.value && selectedType.value !== 'Tous' && selectedType.value !== 'Autre') {
    filtres.type_document = selectedType.value
  }

  const resultat = await listerLivres(filtres)
  if (resultat) livres.value = resultat.livres
}

let debounceTimer: ReturnType<typeof setTimeout> | null = null
watch([searchQuery, selectedType], () => {
  if (debounceTimer) clearTimeout(debounceTimer)
  debounceTimer = setTimeout(chargerLivres, 300)
})

onMounted(chargerLivres)

// ─── Dépôt ────────────────────────────────────────────────────────────────

async function deposer(charge: { formulaire: any, image: File | null, fichier: File | null }) {
  isSubmitting.value = true
  erreurSoumission.value = null

  const nouveauLivre = await creerLivre(charge.formulaire, charge.image, charge.fichier)

  if (nouveauLivre) {
    livres.value.unshift(nouveauLivre)
    modaleDepot.value = false
  }
  else {
    erreurSoumission.value = erreur.value || 'Erreur lors de la création du document.'
  }

  isSubmitting.value = false
}

// ─── Affichage ────────────────────────────────────────────────────────────

function formatDate(dateString: string | null) {
  if (!dateString) return ''
  return new Date(dateString).toLocaleDateString('fr-FR', { month: 'long', year: 'numeric' })
}
</script>

<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        titre="Librafrica"
        sous-titre="Toute la connaissance sur l'Afrique, à portée de clic"
        image="/images/biblio.png"
        aide="C'est quoi Librafrica ?"
        @aide="decouverteOuverte = true"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane :segments="[{ libelle: 'Librafrica' }]">
        <template #action>
          <AfricansBouton icone="fa-solid fa-plus" @click="modaleDepot = true">
            Déposer un document
          </AfricansBouton>
        </template>
      </AfricansFilAriane>
    </template>

    <div class="flex flex-col gap-6">
      <label class="relative block">
        <span class="sr-only">Rechercher un document</span>
        <font-awesome-icon
          icon="fa-solid fa-magnifying-glass"
          class="pointer-events-none absolute top-1/2 left-4 -translate-y-1/2 text-af-atone-2"
        />
        <input
          v-model="searchQuery"
          type="search"
          placeholder="Titre, auteur, mot-clé…"
          class="h-11 w-full rounded-[10px] border border-af-bordure bg-white pr-4 pl-11 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
        />
      </label>

      <div class="flex flex-wrap gap-2">
        <button
          v-for="type in TYPES_DOCUMENT"
          :key="type"
          type="button"
          class="rounded-full px-3 py-1.5 text-[12px]/[1.4] font-bold transition"
          :class="selectedType === type ? 'bg-af-chocolat text-white' : 'bg-af-fond text-af-corps hover:bg-af-bordure'"
          :aria-pressed="selectedType === type"
          @click="selectedType = type"
        >
          {{ type }}
        </button>
      </div>

      <div v-if="chargement" class="grid gap-5 sm:grid-cols-2">
        <div v-for="n in 4" :key="n" class="h-80 animate-pulse rounded-[10px] bg-af-bordure" />
      </div>

      <p v-else-if="erreur" role="alert" class="flex items-center gap-2 rounded-[10px] bg-af-live/10 p-3 text-[14px]/[1.4] text-af-live">
        <font-awesome-icon icon="fa-solid fa-circle-exclamation" />
        {{ erreur }}
      </p>

      <div v-else-if="livres.length" class="grid gap-5 sm:grid-cols-2">
        <article
          v-for="document in livres"
          :key="document.id"
          class="flex flex-col overflow-hidden rounded-[10px] border border-af-bordure bg-white transition hover:border-af-chocolat"
        >
          <button
            type="button"
            class="group relative aspect-[3/4] w-full overflow-hidden bg-af-fond"
            @click="pdfSelected = { url: document.document_pdf_url, acces: document.acces }"
          >
            <img
              v-if="document.image_couverture_url"
              :src="document.image_couverture_url"
              :alt="document.titre"
              class="size-full object-cover transition duration-500 group-hover:scale-105"
            />
            <!-- Repli local : un document sans couverture n'a pas à aller
                 chercher une image chez un hébergeur tiers. -->
            <span v-else class="grid size-full place-items-center">
              <font-awesome-icon icon="fa-solid fa-file-lines" class="text-5xl text-af-atone-2" />
            </span>

            <span class="absolute inset-0 flex items-end justify-center bg-gradient-to-t from-black/80 to-transparent p-4 text-[14px]/[1.4] font-bold text-white opacity-0 transition group-hover:opacity-100">
              Cliquer pour consulter
            </span>
          </button>

          <div class="flex flex-1 flex-col gap-2 p-4">
            <h2 class="line-clamp-2 text-[14px]/[1.4] font-bold text-af-encre">{{ document.titre }}</h2>
            <p v-if="document.date_publication" class="flex items-center gap-1.5 text-[12px]/[1.4] text-af-atone">
              <font-awesome-icon icon="fa-solid fa-calendar-days" />
              {{ formatDate(document.date_publication) }}
            </p>
            <AfricansEtiquette class="mt-auto self-start" :ton="document.acces === 'Lecture' ? 'gris' : 'vert'">
              {{ document.acces === 'Lecture' ? 'Lecture seule' : 'Téléchargeable' }}
            </AfricansEtiquette>
          </div>
        </article>
      </div>

      <div v-else class="rounded-[10px] border border-af-bordure bg-white p-12 text-center">
        <font-awesome-icon icon="fa-solid fa-file-lines" class="text-4xl text-af-atone-2" />
        <p class="mt-4 text-[16px]/[1.4] font-bold">Aucun document trouvé</p>
        <p class="mt-2 text-[14px]/[1.4] text-af-corps">
          Essayez un autre mot-clé ou un autre type de document.
        </p>
      </div>
    </div>

    <template #rail>
      <AfricansPanneau titre="Contribuer" icone="fa-solid fa-file-lines">
        <div class="flex flex-col gap-3">
          <p class="text-[14px]/[1.4] text-af-corps">
            Mémoire, thèse, rapport, étude ou publication : déposez vos travaux pour les rendre
            accessibles au plus grand nombre.
          </p>
          <AfricansBouton icone="fa-solid fa-plus" @click="modaleDepot = true">
            Déposer un document
          </AfricansBouton>
        </div>
      </AfricansPanneau>
    </template>

    <CommonPdfViewer
      v-if="pdfSelected.url"
      :url="pdfSelected.url"
      :acces="pdfSelected.acces"
      @close="pdfSelected = { url: null, acces: null }"
    />

    <BibliothequeProposerDocumentModale
      v-model="modaleDepot"
      :en-cours="isSubmitting"
      :erreur="erreurSoumission"
      @soumettre="deposer"
    />

    <BibliothequeDecouverteLibrafrica v-model="decouverteOuverte" />
  </NuxtLayout>
</template>
