<script setup lang="ts">
import { useUserStore } from '~/stores/user'
import { formatDate } from '~/composables/useOpportuniteAfrique'

/**
 * Mes contributions : portée sur le gabarit de la refonte.
 *
 * La page était restée sur l'ancienne enveloppe (barre à méga-menus, pas de
 * navigation latérale). Trois autres défauts sont corrigés au passage :
 *
 *   - `recette_culinaire` s'affichait BRUT, l'enum n'ayant jamais été ajouté à
 *     la table de libellés : de même pour `modification`, que le SQL emploie
 *     alors que la table ne connaissait que `edition` ;
 *   - toutes les lignes d'un même territoire se ressemblaient : « Côte d'Ivoire ·
 *     Personnalité connue · Modification », cinq fois, sans moyen de les
 *     distinguer. L'API renvoie désormais `libelle_objet`, tiré du payload ;
 *   - les filtres occupaient une barre pleine largeur au-dessus de la liste ;
 *     ils passent dans le rail, avec le décompte par état.
 */
definePageMeta({ middleware: 'auth', layout: false })

useHead({ title: 'Mes contributions | AfricanS' })

interface MaContribution {
  id: string
  fiche_pays_id: string
  pays_nom: string | null
  type_objet_contribution: string
  section_afripulse: string | null
  type_contribution: string
  etat: string
  note_moderation: string | null
  libelle_objet: string | null
  created_at: string
  traite_at: string | null
}

interface ReponseAPI {
  success: boolean
  data: { contributions: MaContribution[], total: number } | null
  error: string | null
}

const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string
const userStore = useUserStore()

const contributions = ref<MaContribution[]>([])
const chargement = ref(false)
const total = ref(0)
const page = ref(1)
const parPage = ref(20)

const filtres = reactive({ etat: '', type_objet: '' })

const totalPages = computed(() => Math.max(1, Math.ceil(total.value / parPage.value)))
const filtreActif = computed(() => Boolean(filtres.etat || filtres.type_objet))

const charger = async () => {
  chargement.value = true
  try {
    const params = new URLSearchParams({
      page: String(page.value),
      par_page: String(parPage.value),
    })
    if (filtres.etat) params.set('etat', filtres.etat)
    if (filtres.type_objet) params.set('type_objet', filtres.type_objet)

    const reponse = await $fetch<ReponseAPI>(
      `${apiBase}/api/fiches-pays/moi/contributions?${params}`,
      { headers: { Authorization: `Bearer ${userStore.accessToken}` } },
    )
    if (reponse.success && reponse.data) {
      contributions.value = reponse.data.contributions
      total.value = reponse.data.total
    }
  }
  catch (e) {
    console.error('Erreur chargement contributions:', e)
  }
  finally {
    chargement.value = false
  }
}

const rafraichir = () => {
  page.value = 1
  charger()
}

const reinitialiser = () => {
  filtres.etat = ''
  filtres.type_objet = ''
  rafraichir()
}

const allerPage = (n: number) => {
  if (n < 1 || n > totalPages.value) return
  page.value = n
  charger()
}

onMounted(charger)

// ─── Libellés ─────────────────────────────────────────────────────────────

const ETATS = [
  { valeur: 'en_attente', libelle: 'En attente', icone: 'fa-solid fa-clock', ton: 'bg-af-chocolat/10 text-af-chocolat' },
  { valeur: 'approuvee', libelle: 'Approuvée', icone: 'fa-solid fa-circle-check', ton: 'bg-af-vert/10 text-af-vert' },
  { valeur: 'refusee', libelle: 'Refusée', icone: 'fa-solid fa-circle-xmark', ton: 'bg-af-live/10 text-af-live' },
  { valeur: 'rejetee', libelle: 'Refusée', icone: 'fa-solid fa-circle-xmark', ton: 'bg-af-live/10 text-af-live' },
  { valeur: 'obsolete', libelle: 'Obsolète', icone: 'fa-solid fa-ban', ton: 'bg-af-fond text-af-atone' }]
const etat = (e: string) => ETATS.find(x => x.valeur === e)
  ?? { valeur: e, libelle: e, icone: 'fa-solid fa-circle', ton: 'bg-af-fond text-af-corps' }

/** Les huit `type_objet_contribution` de l'enum SQL, aucun ne doit tomber
 *  en repli sur sa valeur brute, comme le faisait `recette_culinaire`. */
const TYPES_OBJET: Record<string, string> = {
  fiche_pays: 'Nouvelle fiche territoire',
  site_touristique: 'Site touristique',
  secteur_developpement: 'Secteur d\'opportunité',
  personnalite_connue: 'Personnalité connue',
  savoir_pratique: 'Savoir pratique',
  recommandation_visiteur: 'Recommandation',
  photo_visiteur: 'Photo',
  recette_culinaire: 'Recette culinaire',
}
const libelleTypeObjet = (t: string) => TYPES_OBJET[t] ?? t

/** L'enum SQL dit « modification » ; le code interne disait « edition ». Les
 *  deux graphies circulent, les deux sont traduites. */
const TYPES_CONTRIBUTION: Record<string, string> = {
  ajout: 'Ajout',
  edition: 'Modification',
  modification: 'Modification',
  suppression: 'Suppression',
}
const libelleTypeContribution = (t: string) => TYPES_CONTRIBUTION[t] ?? t
</script>

<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        titre="Mes contributions"
        sous-titre="Suivez l'état de vos propositions sur les fiches territoire Afripulse"
        image="/images/africans/heros/hero-afripulse.jpg"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[{ libelle: 'Mon compte', vers: '/mon-compte/profil' }, { libelle: 'Mes contributions' }]"
      >
        <template #action>
          <AfricansBouton icone="fa-solid fa-earth-africa" vers="/opportunite-afrique">
            Voir les territoires
          </AfricansBouton>
        </template>
      </AfricansFilAriane>
    </template>

    <div class="flex flex-col gap-5">
      <p v-if="!chargement" class="text-[14px]/[1.4] text-af-atone">
        <span class="font-bold text-af-encre">{{ total }}</span>
        contribution{{ total > 1 ? 's' : '' }}
        <span v-if="filtreActif">(filtré{{ total > 1 ? 'es' : 'e' }})</span>
      </p>

      <div v-if="chargement" class="flex flex-col gap-4">
        <div v-for="n in 4" :key="n" class="h-24 animate-pulse rounded-[10px] bg-af-bordure" />
      </div>

      <div v-else-if="!contributions.length" class="rounded-[10px] border border-af-bordure bg-white p-12 text-center">
        <font-awesome-icon icon="fa-solid fa-clipboard-list" class="text-4xl text-af-atone-2" />
        <p class="mt-4 text-[16px]/[1.4] font-bold">
          {{ filtreActif ? 'Aucune contribution ne correspond' : 'Aucune contribution pour le moment' }}
        </p>
        <p class="mt-2 text-[14px]/[1.4] text-af-corps">
          Enrichissez une fiche territoire : recette, site, personnalité, savoir pratique…
        </p>
        <AfricansBouton class="mt-6" icone="fa-solid fa-earth-africa" vers="/opportunite-afrique">
          Découvrir les territoires
        </AfricansBouton>
      </div>

      <template v-else>
        <article
          v-for="c in contributions"
          :key="c.id"
          class="flex flex-col gap-3 rounded-[10px] border border-af-bordure bg-white p-5 transition hover:border-af-chocolat"
        >
          <div class="flex flex-wrap items-start justify-between gap-3">
            <div class="flex min-w-0 flex-col gap-2">
              <!-- Le libellé de l'objet AVANT le territoire : c'est lui qui
                   distingue deux lignes d'une même fiche. -->
              <p class="text-[17px]/[1.4] font-bold text-af-encre">
                {{ c.libelle_objet || libelleTypeObjet(c.type_objet_contribution) }}
              </p>
              <p class="flex flex-wrap items-center gap-x-3 gap-y-1 text-[12px]/[1.4] text-af-atone">
                <span
                  class="inline-flex items-center gap-1.5 rounded-full px-2.5 py-0.5 font-bold"
                  :class="etat(c.etat).ton"
                >
                  <font-awesome-icon :icon="etat(c.etat).icone" />
                  {{ etat(c.etat).libelle }}
                </span>
                <span>{{ libelleTypeObjet(c.type_objet_contribution) }}</span>
                <span>·</span>
                <span>{{ libelleTypeContribution(c.type_contribution) }}</span>
                <NuxtLink
                  v-if="c.pays_nom"
                  :to="`/opportunite-afrique/${c.fiche_pays_id}`"
                  class="flex items-center gap-1.5 font-bold text-af-chocolat transition hover:opacity-70"
                >
                  <font-awesome-icon icon="fa-solid fa-location-dot" />
                  {{ c.pays_nom }}
                </NuxtLink>
                <span v-else>Nouvelle fiche territoire</span>
              </p>
            </div>

            <p class="shrink-0 text-right text-[12px]/[1.4] text-af-atone">
              Soumise le {{ formatDate(c.created_at) }}
              <span v-if="c.traite_at" class="block">Traitée le {{ formatDate(c.traite_at) }}</span>
            </p>
          </div>

          <p
            v-if="c.note_moderation"
            class="rounded-[10px] border border-af-live/30 bg-af-live/5 p-3 text-[12px]/[1.4] text-af-live"
          >
            <span class="font-bold">Motif :</span> {{ c.note_moderation }}
          </p>
        </article>

        <nav v-if="totalPages > 1" class="flex items-center justify-center gap-3">
          <AfricansBouton variante="secondaire" :desactive="page <= 1" @click="allerPage(page - 1)">
            Précédent
          </AfricansBouton>
          <span class="text-[14px]/[1.4] text-af-corps">{{ page }} / {{ totalPages }}</span>
          <AfricansBouton variante="secondaire" :desactive="page >= totalPages" @click="allerPage(page + 1)">
            Suivant
          </AfricansBouton>
        </nav>
      </template>
    </div>

    <template #rail>
      <AfricansPanneau titre="Filtres" icone="fa-solid fa-sliders" action-libelle="Réinitialiser" @action="reinitialiser">
        <div class="flex flex-col gap-5">
          <label class="flex flex-col gap-2">
            <span class="text-[12px]/[1.4] font-bold text-af-atone uppercase">État</span>
            <select
              v-model="filtres.etat"
              class="h-10 w-full rounded-[10px] border border-af-bordure bg-white px-3 text-[14px]/[1.4] focus:outline-2 focus:outline-af-chocolat"
              @change="rafraichir"
            >
              <option value="">Tous les états</option>
              <option value="en_attente">En attente</option>
              <option value="approuvee">Approuvée</option>
              <option value="refusee">Refusée</option>
              <option value="obsolete">Obsolète / retirée</option>
            </select>
          </label>

          <label class="flex flex-col gap-2">
            <span class="text-[12px]/[1.4] font-bold text-af-atone uppercase">Type</span>
            <select
              v-model="filtres.type_objet"
              class="h-10 w-full rounded-[10px] border border-af-bordure bg-white px-3 text-[14px]/[1.4] focus:outline-2 focus:outline-af-chocolat"
              @change="rafraichir"
            >
              <option value="">Tous les types</option>
              <option v-for="(libelle, cle) in TYPES_OBJET" :key="cle" :value="cle">{{ libelle }}</option>
            </select>
          </label>
        </div>
      </AfricansPanneau>

      <AfricansPanneau titre="Comment ça marche" icone="fa-solid fa-circle-info">
        <p class="text-[14px]/[1.4] text-af-corps">
          Chaque proposition est examinée par un administrateur avant publication. Une contribution
          <strong class="font-bold text-af-encre">obsolète</strong> a été devancée par une autre
          modification du même élément : reproposez-la depuis la fiche.
        </p>
      </AfricansPanneau>

      <ComptePanneauNavigation />
    </template>
  </NuxtLayout>
</template>
