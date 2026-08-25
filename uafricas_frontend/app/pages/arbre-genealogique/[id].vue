<script setup lang="ts">
definePageMeta({ middleware: 'auth' })

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
</script>

<template>
  <div class="mt-28">
    <!-- Hero Section (compact, titre ↔ description au survol) -->
    <div class="group bg-gradient-to-br from-[var(--color-custom-green)]/10 via-white to-[var(--color-custom-chocolat)]/5 px-4 pt-6 pb-6 text-center select-none">
      <div class="mx-auto max-w-2xl">
        <div class="mx-auto mb-3 flex h-14 w-14 items-center justify-center rounded-full bg-[var(--color-custom-green)]/20">
          <font-awesome-icon :icon="['fas', 'user']" class="text-xl text-[var(--color-custom-green)]" />
        </div>
        <!-- Conteneur fixe : le titre et la description se superposent (crossfade au survol) -->
        <div class="relative flex items-center justify-center min-h-10 md:min-h-12">
          <h1 class="absolute inset-0 flex items-center justify-center text-stone-800 text-2xl font-bold transition-opacity duration-300 group-hover:opacity-0">
            Fiche personne
          </h1>
          <p class="absolute inset-0 flex items-center justify-center text-stone-500 text-sm px-2 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
            Consultez et modifiez les informations de cette personne
          </p>
        </div>
      </div>
    </div>

    <div class="max-w-3xl mx-auto px-4 py-8">
    <!-- Navigation -->
    <NuxtLink
      to="/arbre-genealogique"
      class="inline-flex items-center gap-2 text-sm text-stone-500 hover:text-custom-chocolat transition-colors mb-6"
    >
      <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
      </svg>
      Retour à l'arbre
    </NuxtLink>

    <!-- Chargement -->
    <div v-if="chargement" class="flex justify-center py-20">
      <div class="w-8 h-8 border-4 border-custom-chocolat/30 border-t-custom-chocolat rounded-full animate-spin" />
    </div>

    <!-- 404 -->
    <div v-else-if="erreur404" class="text-center py-20">
      <p class="text-2xl font-bold text-stone-700 mb-2">Personne introuvable</p>
      <p class="text-stone-500 mb-6">Cette fiche n'existe pas ou ne fait pas partie de votre arbre.</p>
      <NuxtLink to="/arbre-genealogique" class="text-custom-chocolat hover:underline">
        Retour à l'arbre
      </NuxtLink>
    </div>

    <!-- Contenu -->
    <div v-else-if="detail">
      <!-- En-tête -->
      <div class="bg-white border border-stone-200 rounded-2xl p-6 mb-6">
        <div class="flex items-start gap-4">
          <!-- Photo / initiales -->
          <div class="shrink-0">
            <img
              v-if="detail.personne.photo_url"
              :src="detail.personne.photo_url"
              :alt="nomComplet"
              class="w-20 h-20 rounded-full object-cover border-2 border-stone-200"
            />
            <div
              v-else
              class="w-20 h-20 rounded-full bg-custom-chocolat/10 border-2 border-custom-chocolat/20 flex items-center justify-center"
            >
              <span class="text-xl font-bold text-custom-chocolat">
                {{ detail.personne.prenoms?.charAt(0) ?? '' }}{{ detail.personne.nom.charAt(0) }}
              </span>
            </div>
          </div>

          <!-- Infos -->
          <div class="flex-1 min-w-0">
            <h1 class="text-2xl font-bold text-stone-800 leading-tight">{{ nomComplet }}</h1>
            <p v-if="libelleGenre(detail.personne.genre)" class="text-sm text-stone-500 mt-1">
              {{ libelleGenre(detail.personne.genre) }}
            </p>
          </div>

          <!-- Actions (hors édition) -->
          <div v-if="!modeEdition" class="flex gap-2 shrink-0">
            <button
              class="px-4 py-2 text-sm font-semibold border border-stone-300 text-stone-700 rounded-lg hover:bg-stone-50 transition-colors"
              @click="modeEdition = true"
            >
              Modifier
            </button>
            <button
              class="px-4 py-2 text-sm font-semibold border border-red-200 text-red-600 rounded-lg hover:bg-red-50 transition-colors"
              @click="confirmerSuppression = true"
            >
              Supprimer
            </button>
          </div>
        </div>

        <!-- Détails biographiques -->
        <div v-if="!modeEdition" class="mt-4 grid grid-cols-2 gap-4 text-sm">
          <div v-if="detail.personne.naissance">
            <span class="text-stone-500">Naissance :</span>
            <span class="ml-1 text-stone-800">{{ dateNaissance }}</span>
            <span v-if="detail.personne.naissance_lieu" class="ml-1 text-stone-500">
              {{ detail.personne.naissance_lieu }}
            </span>
          </div>
          <div v-if="detail.personne.deces">
            <span class="text-stone-500">Décès :</span>
            <span class="ml-1 text-stone-800">{{ dateDeces }}</span>
            <span v-if="detail.personne.deces_lieu" class="ml-1 text-stone-500">
              {{ detail.personne.deces_lieu }}
            </span>
          </div>
        </div>

        <!-- Formulaire d'édition -->
        <div v-if="modeEdition" class="mt-4 border-t border-stone-100 pt-4">
          <PersonneForm
            :model-value="formEdition"
            :loading="enregistrement"
            mode-edition
            @submit="sauvegarder"
            @annuler="modeEdition = false"
          />
        </div>
      </div>

      <!-- Confirmation suppression -->
      <div
        v-if="confirmerSuppression"
        class="bg-red-50 border border-red-200 rounded-xl p-4 mb-6"
      >
        <p class="text-sm font-semibold text-red-700 mb-3">
          Confirmer la suppression de {{ nomComplet }} ?
        </p>
        <p class="text-xs text-red-600 mb-3">
          Cette action supprimera la personne de votre arbre et tous ses liens familiaux associés.
        </p>
        <div class="flex gap-2">
          <button
            :disabled="suppression"
            class="px-4 py-2 bg-red-600 text-white text-sm font-semibold rounded-lg hover:bg-red-700 transition-colors disabled:opacity-50"
            @click="confirmerEtSupprimer"
          >
            <span v-if="suppression">Suppression…</span>
            <span v-else>Oui, supprimer</span>
          </button>
          <button
            class="px-4 py-2 border border-stone-300 text-stone-700 text-sm font-semibold rounded-lg hover:bg-stone-50 transition-colors"
            @click="confirmerSuppression = false"
          >
            Annuler
          </button>
        </div>
      </div>

      <!-- Section liens familiaux -->
      <div class="space-y-4">
        <!-- Parents -->
        <section class="bg-white border border-stone-200 rounded-xl p-5">
          <h2 class="text-base font-bold text-stone-700 mb-3">Parents</h2>
          <div v-if="detail.parents.length > 0" class="space-y-2">
            <div
              v-for="lien in detail.parents"
              :key="lien.lien_id"
              class="flex items-center justify-between gap-3 p-3 bg-stone-50 rounded-lg"
            >
              <div class="flex items-center gap-2 min-w-0">
                <span class="text-xs font-medium text-custom-chocolat bg-custom-chocolat/10 px-2 py-0.5 rounded-full shrink-0">
                  {{ libelleLien(lien.type_lien as TypeLien) }}
                </span>
                <NuxtLink
                  :to="`/arbre-genealogique/${lien.personne.id}`"
                  class="text-sm font-medium text-stone-800 hover:text-custom-chocolat transition-colors truncate"
                >
                  {{ lien.personne.prenoms ? `${lien.personne.prenoms} ${lien.personne.nom}` : lien.personne.nom }}
                </NuxtLink>
              </div>
              <button
                class="shrink-0 text-xs text-stone-400 hover:text-red-500 transition-colors"
                @click="supprimerUnLien(lien.lien_id)"
              >
                Retirer
              </button>
            </div>
          </div>
          <p v-else class="text-sm text-stone-400 italic">Aucun parent enregistré</p>
        </section>

        <!-- Enfants -->
        <section class="bg-white border border-stone-200 rounded-xl p-5">
          <h2 class="text-base font-bold text-stone-700 mb-3">Enfants</h2>
          <div v-if="detail.enfants.length > 0" class="space-y-2">
            <div
              v-for="lien in detail.enfants"
              :key="lien.lien_id"
              class="flex items-center justify-between gap-3 p-3 bg-stone-50 rounded-lg"
            >
              <div class="flex items-center gap-2 min-w-0">
                <span class="text-xs font-medium text-custom-green/90 bg-custom-green/10 px-2 py-0.5 rounded-full shrink-0">
                  Enfant
                </span>
                <NuxtLink
                  :to="`/arbre-genealogique/${lien.personne.id}`"
                  class="text-sm font-medium text-stone-800 hover:text-custom-chocolat transition-colors truncate"
                >
                  {{ lien.personne.prenoms ? `${lien.personne.prenoms} ${lien.personne.nom}` : lien.personne.nom }}
                </NuxtLink>
              </div>
              <button
                class="shrink-0 text-xs text-stone-400 hover:text-red-500 transition-colors"
                @click="supprimerUnLien(lien.lien_id)"
              >
                Retirer
              </button>
            </div>
          </div>
          <p v-else class="text-sm text-stone-400 italic">Aucun enfant enregistré</p>
        </section>

        <!-- Conjoints -->
        <section class="bg-white border border-stone-200 rounded-xl p-5">
          <h2 class="text-base font-bold text-stone-700 mb-3">Conjoints</h2>
          <div v-if="detail.conjoints.length > 0" class="space-y-2">
            <div
              v-for="lien in detail.conjoints"
              :key="lien.lien_id"
              class="flex items-center justify-between gap-3 p-3 bg-stone-50 rounded-lg"
            >
              <div class="flex items-center gap-2 min-w-0">
                <span class="text-xs font-medium text-stone-500 bg-stone-100 px-2 py-0.5 rounded-full shrink-0">
                  Conjoint(e)
                </span>
                <NuxtLink
                  :to="`/arbre-genealogique/${lien.personne.id}`"
                  class="text-sm font-medium text-stone-800 hover:text-custom-chocolat transition-colors truncate"
                >
                  {{ lien.personne.prenoms ? `${lien.personne.prenoms} ${lien.personne.nom}` : lien.personne.nom }}
                </NuxtLink>
              </div>
              <button
                class="shrink-0 text-xs text-stone-400 hover:text-red-500 transition-colors"
                @click="supprimerUnLien(lien.lien_id)"
              >
                Retirer
              </button>
            </div>
          </div>
          <p v-else class="text-sm text-stone-400 italic">Aucun conjoint enregistré</p>
        </section>

        <!-- Ajouter un lien -->
        <section class="bg-white border border-stone-200 rounded-xl p-5">
          <div v-if="!afficherFormLien" class="flex items-center justify-between">
            <h2 class="text-base font-bold text-stone-700">Ajouter un lien familial</h2>
            <button
              class="px-4 py-2 text-sm font-semibold bg-custom-chocolat text-white rounded-lg hover:opacity-90 transition-opacity"
              @click="afficherFormLien = true"
            >
              + Ajouter
            </button>
          </div>
          <div v-else>
            <h2 class="text-base font-bold text-stone-700 mb-4">Nouveau lien familial</h2>
            <LienFamilialForm
              ref="refFormLien"
              :rattachement-source-id="detail.rattachement_id"
              :loading="creationLien"
              @submit="soumettreLien"
              @annuler="afficherFormLien = false"
            />
          </div>
        </section>
      </div>
    </div>
    </div>
  </div>
</template>
