<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        titre="Rootstree"
        :sous-titre="detail ? nomComplet : 'Fiche d\'une personne de votre arbre'"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[{ libelle: 'Rootstree', vers: '/arbre-genealogique' }, { libelle: detail ? nomComplet : 'Fiche' }]"
      >
        <template #action>
          <AfricansBouton icone="fa-solid fa-diagram-project" vers="/arbre-genealogique/visualisation">
            Voir mon arbre
          </AfricansBouton>
        </template>
      </AfricansFilAriane>
    </template>

    <div v-if="chargement" class="flex flex-col gap-5">
      <div v-for="n in 3" :key="n" class="h-32 animate-pulse rounded-[10px] bg-af-bordure" />
    </div>

    <div v-else-if="erreur404" class="rounded-[10px] border border-af-bordure bg-white p-12 text-center">
      <font-awesome-icon icon="fa-solid fa-user-slash" class="text-4xl text-af-atone-2" />
      <p class="mt-4 text-[16px]/[1.4] font-bold">Personne introuvable</p>
      <p class="mt-2 text-[14px]/[1.4] text-af-corps">
        Cette fiche n'existe pas ou ne fait pas partie de votre arbre.
      </p>
      <AfricansBouton class="mt-6" variante="secondaire" icone="fa-solid fa-arrow-left" vers="/arbre-genealogique">
        Retour à l'arbre
      </AfricansBouton>
    </div>

    <div v-else-if="detail" class="flex flex-col gap-5">
      <!-- Identité -->
      <div class="flex flex-col gap-5 rounded-[10px] border border-af-bordure bg-white p-6">
        <div class="flex items-start gap-4">
          <img
            v-if="urlMedia(detail.personne.photo_url)"
            :src="urlMedia(detail.personne.photo_url)!"
            :alt="nomComplet"
            class="size-20 shrink-0 rounded-full object-cover"
          />
          <span
            v-else
            class="grid size-20 shrink-0 place-items-center rounded-full bg-af-chocolat/15 text-[20px]/[1.4] font-bold text-af-chocolat"
          >
            {{ detail.personne.prenoms?.charAt(0) ?? '' }}{{ detail.personne.nom.charAt(0) }}
          </span>

          <div class="min-w-0 flex-1">
            <h1 class="text-[20px]/[1.4] font-bold text-af-encre">{{ nomComplet }}</h1>
            <p v-if="libelleGenre(detail.personne.genre)" class="mt-1 text-[14px]/[1.4] text-af-atone">
              {{ libelleGenre(detail.personne.genre) }}
            </p>
          </div>
        </div>

        <dl v-if="!modeEdition" class="grid gap-4 sm:grid-cols-2">
          <div v-if="detail.personne.naissance">
            <dt class="text-[12px]/[1.4] text-af-atone">Naissance</dt>
            <dd class="text-[14px]/[1.4] text-af-encre">
              {{ dateNaissance }}
              <span v-if="detail.personne.naissance_lieu" class="text-af-atone">
                {{ detail.personne.naissance_lieu }}
              </span>
            </dd>
          </div>
          <div v-if="detail.personne.deces">
            <dt class="text-[12px]/[1.4] text-af-atone">Décès</dt>
            <dd class="text-[14px]/[1.4] text-af-encre">
              {{ dateDeces }}
              <span v-if="detail.personne.deces_lieu" class="text-af-atone">
                {{ detail.personne.deces_lieu }}
              </span>
            </dd>
          </div>
        </dl>

        <div v-if="modeEdition" class="border-t border-af-bordure pt-5">
          <PersonneForm
            :model-value="formEdition"
            :loading="enregistrement"
            mode-edition
            @submit="sauvegarder"
            @annuler="modeEdition = false"
          />
        </div>
      </div>

      <!-- La confirmation reste EN LIGNE et non en modale : elle énonce une
           conséquence en cascade (les liens familiaux partent avec), qu'il faut
           pouvoir relire à côté de la fiche qu'on s'apprête à supprimer. -->
      <div
        v-if="confirmerSuppression"
        class="flex flex-col gap-3 rounded-[10px] border border-af-live/30 bg-af-live/5 p-5"
      >
        <p class="text-[14px]/[1.4] font-bold text-af-live">
          Confirmer la suppression de {{ nomComplet }} ?
        </p>
        <p class="text-[12px]/[1.4] text-af-corps">
          Cette action supprimera la personne de votre arbre et tous ses liens familiaux associés.
        </p>
        <div class="flex flex-wrap gap-3">
          <AfricansBouton
            :desactive="suppression"
            :tourne="suppression"
            icone="fa-solid fa-trash"
            @click="supprimer"
          >
            {{ suppression ? 'Suppression…' : 'Supprimer définitivement' }}
          </AfricansBouton>
          <button
            type="button"
            class="text-base font-bold text-af-corps transition hover:opacity-70"
            @click="confirmerSuppression = false"
          >
            Annuler
          </button>
        </div>
      </div>

      <!-- Liens familiaux : trois sections de même forme -->
      <AfricansAccordeon
        v-for="groupe in groupesLiens"
        :key="groupe.cle"
        :titre="`${groupe.titre} (${groupe.liens.length})`"
        :icone="groupe.icone"
        fond="blanc"
        par-defaut-ouvert
      >
        <ul v-if="groupe.liens.length" class="flex flex-col gap-2">
          <li
            v-for="lien in groupe.liens"
            :key="lien.lien_id"
            class="flex items-center justify-between gap-3 rounded-[10px] bg-af-fond p-3"
          >
            <div class="flex min-w-0 items-center gap-3">
              <AfricansEtiquette>{{ groupe.libelle(lien) }}</AfricansEtiquette>
              <NuxtLink
                :to="`/arbre-genealogique/${lien.personne.id}`"
                class="min-w-0 truncate text-[14px]/[1.4] font-bold text-af-encre transition hover:text-af-chocolat"
              >
                {{ lien.personne.prenoms ? `${lien.personne.prenoms} ${lien.personne.nom}` : lien.personne.nom }}
              </NuxtLink>
            </div>
            <button
              type="button"
              class="shrink-0 text-[12px]/[1.4] font-bold text-af-corps transition hover:text-af-live"
              @click="supprimerUnLien(lien.lien_id)"
            >
              Retirer
            </button>
          </li>
        </ul>
        <p v-else class="text-[14px]/[1.4] text-af-atone italic">{{ groupe.vide }}</p>
      </AfricansAccordeon>

      <!-- Ajout d'un lien -->
      <div class="rounded-[10px] border border-af-bordure bg-white p-6">
        <div v-if="!afficherFormLien" class="flex flex-wrap items-center justify-between gap-3">
          <h2 class="text-[17px]/[1.4] font-bold text-af-encre">Ajouter un lien familial</h2>
          <AfricansBouton icone="fa-solid fa-plus" @click="afficherFormLien = true">
            Ajouter
          </AfricansBouton>
        </div>
        <div v-else class="flex flex-col gap-4">
          <h2 class="text-[17px]/[1.4] font-bold text-af-encre">Nouveau lien familial</h2>
          <LienFamilialForm
            ref="refFormLien"
            :rattachement-source-id="detail.rattachement_id"
            :loading="creationLien"
            @submit="soumettreLien"
            @annuler="afficherFormLien = false"
          />
        </div>
      </div>
    </div>

    <template #rail>
      <AfricansPanneau v-if="detail && !modeEdition" titre="Cette fiche" icone="fa-solid fa-user">
        <div class="flex flex-col gap-3">
          <AfricansBouton variante="secondaire" icone="fa-solid fa-pen-to-square" pleine-largeur @click="modeEdition = true">
            Modifier
          </AfricansBouton>
          <AfricansBouton variante="secondaire" icone="fa-solid fa-trash" pleine-largeur @click="confirmerSuppression = true">
            Supprimer
          </AfricansBouton>
        </div>
      </AfricansPanneau>

      <AfricansPanneau titre="Mon arbre" icone="fa-solid fa-sitemap">
        <div class="flex flex-col gap-3">
          <AfricansBouton variante="secondaire" icone="fa-solid fa-diagram-project" pleine-largeur vers="/arbre-genealogique/visualisation">
            Visualisation
          </AfricansBouton>
          <AfricansBouton variante="secondaire" icone="fa-solid fa-arrow-left" pleine-largeur vers="/arbre-genealogique">
            Toutes les personnes
          </AfricansBouton>
        </div>
      </AfricansPanneau>
    </template>
  </NuxtLayout>
</template>

<script setup lang="ts">
definePageMeta({ middleware: 'auth', layout: false })

import type { PersonneDetail, LienResume, TypeLien } from '~/mocks/arbre-genealogique'
import { formaterDate } from '~/mocks/arbre-genealogique'
import { useArbreGenealogique } from '~/composables/useArbreGenealogique'

const route = useRoute()
const router = useRouter()
const { obtenirPersonne, modifierPersonne, supprimerPersonne, creerLien, supprimerLien } = useArbreGenealogique()

// ─── État ─────────────────────────────────────────────────────────────────

const detail = ref<PersonneDetail | null>(null)
const chargement = ref(true)
const erreur404 = ref(false)

const modeEdition = ref(false)
const enregistrement = ref(false)
const suppression = ref(false)
const confirmerSuppression = ref(false)

const afficherFormLien = ref(false)
const creationLien = ref(false)
const erreurLien = ref('')

const refFormLien = ref<{ afficherErreur: (msg: string) => void } | null>(null)

// ─── Chargement ────────────────────────────────────────────────────────────

async function charger() {
  chargement.value = true
  erreur404.value = false
  try {
    const res = await obtenirPersonne(route.params.id as string)
    if (res.success && res.data) {
      detail.value = res.data
    } else {
      erreur404.value = true
    }
  } catch {
    erreur404.value = true
  } finally {
    chargement.value = false
  }
}

onMounted(charger)

// ─── Infos calculées ──────────────────────────────────────────────────────

const nomComplet = computed(() => {
  if (!detail.value) return ''
  const p = detail.value.personne
  return p.prenoms ? `${p.prenoms} ${p.nom}` : p.nom
})

const dateNaissance = computed(() =>
  detail.value ? formaterDate(detail.value.personne.naissance) : ''
)
const dateDeces = computed(() =>
  detail.value ? formaterDate(detail.value.personne.deces) : ''
)

function libelleGenre(genre?: string) {
  const map: Record<string, string> = {
    masculin: 'Masculin',
    feminin: 'Féminin',
    autre: 'Autre',
    non_precise: 'Non précisé',
  }
  return genre ? map[genre] ?? genre : null
}

function libelleLien(type: TypeLien) {
  const map: Record<TypeLien, string> = {
    pere: 'Père',
    mere: 'Mère',
    parent: 'Parent',
    conjoint: 'Conjoint(e)',
  }
  return map[type]
}

// ─── Édition ──────────────────────────────────────────────────────────────

const formEdition = computed(() => {
  if (!detail.value) return undefined
  const p = detail.value.personne
  return {
    nom: p.nom,
    prenoms: p.prenoms,
    genre: p.genre as any,
    naissance: p.naissance,
    naissance_lieu: p.naissance_lieu,
    deces: p.deces,
    deces_lieu: p.deces_lieu,
  }
})

async function sauvegarder(form: any) {
  if (!detail.value) return
  enregistrement.value = true
  try {
    const res = await modifierPersonne(detail.value.personne.id, form)
    if (res.success && res.data) {
      detail.value = res.data
      modeEdition.value = false
    }
  } finally {
    enregistrement.value = false
  }
}

// ─── Suppression personne ─────────────────────────────────────────────────

async function confirmerEtSupprimer() {
  if (!detail.value) return
  suppression.value = true
  try {
    const res = await supprimerPersonne(detail.value.personne.id)
    if (res.success) {
      await router.push('/arbre-genealogique')
    }
  } finally {
    suppression.value = false
    confirmerSuppression.value = false
  }
}

// ─── Liens familiaux ──────────────────────────────────────────────────────

async function soumettreLien(form: any) {
  creationLien.value = true
  erreurLien.value = ''
  try {
    const res = await creerLien(form)
    if (res.success) {
      afficherFormLien.value = false
      await charger()
    } else if (res.error) {
      refFormLien.value?.afficherErreur(res.error)
    }
  } catch (e: any) {
    const msg = e?.data?.error ?? 'Erreur lors de la création du lien'
    refFormLien.value?.afficherErreur(msg)
  } finally {
    creationLien.value = false
  }
}

async function supprimerUnLien(lienId: string) {
  try {
    const res = await supprimerLien(lienId)
    if (res.success) await charger()
  } catch {
    // silencieux
  }
}

/**
 * Parents, enfants et conjoints : trois sections dont le gabarit était recopié
 * à l'identique, à l'étiquette et à la couleur près. Une seule forme, trois
 * jeux de données.
 */
const groupesLiens = computed(() => {
  const d = detail.value
  if (!d) return []
  return [
    {
      cle: 'parents',
      titre: 'Parents',
      icone: 'fa-solid fa-users',
      liens: d.parents,
      vide: 'Aucun parent enregistré',
      libelle: (lien: any) => libelleLien(lien.type_lien as TypeLien),
    },
    {
      cle: 'enfants',
      titre: 'Enfants',
      icone: 'fa-solid fa-user-plus',
      liens: d.enfants,
      vide: 'Aucun enfant enregistré',
      libelle: () => 'Enfant',
    },
    {
      cle: 'conjoints',
      titre: 'Conjoints',
      icone: 'fa-solid fa-heart',
      liens: d.conjoints,
      vide: 'Aucun conjoint enregistré',
      libelle: (lien: any) => libelleLien(lien.type_lien as TypeLien),
    }]
})
</script>


