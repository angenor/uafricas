<script setup lang="ts">
/**
 * Championship panafricain (feature 013) : les trois classements d'une saison.
 *
 *  - tous les membres ;
 *  - les membres du pays du lecteur (son pays de rattachement) ;
 *  - les pays entre eux.
 *
 * Consultable sans connexion. Un membre connecté voit en plus sa position, même
 * hors des premières places.
 *
 * Deux vues des pays : le classement, et la carte de l'Afrique. La carte ne
 * fait que lire le classement des pays ; choisir un pays ouvre sa fiche dans le
 * rail, d'où l'on peut lancer une partie qui ne porte que sur lui. La liste
 * reste la vue par défaut : la carte n'a pas de survol au doigt, et sur un
 * écran étroit la même information se lit mieux en liste (FR-068).
 */
import { messageErreurJeu, type MonJeuAPI } from '~/composables/useJeu'
import { PALIERS_RANG, COULEUR_SANS_JOUEUR, couleurRangPays } from '~/utils/carteAfrique'
import type {
  ClassementMembresAPI,
  LignePaysAPI,
  SaisonAPI,
  SaisonsAPI,
} from '~/composables/useChampionship'

definePageMeta({ layout: false })

useHead({ title: 'Championship | Activités | AfricanS' })

const { saisons: chargerSaisons, classementMembres, classementPays } = useChampionship()
const { monJeu, creerPartie } = useJeu()
const { redirigerVersConnexion } = useAuth()
const userStore = useUserStore()

// ── Carte ────────────────────────────────────────────────────────────────────
const vue = ref<'classement' | 'carte'>('classement')
const isoChoisi = ref<string | null>(null)
const ficheRef = ref<HTMLElement | null>(null)
const lancementPays = ref(false)
const erreurPays = ref('')

/** Valeur portée par chaque pays sur la carte : son score de saison. */
const scoresParIso = computed<Record<string, number>>(() =>
  Object.fromEntries(pays.value.filter(p => p.iso2).map(p => [p.iso2!, p.score])),
)
const paysParIso = computed(() => new Map(pays.value.filter(p => p.iso2).map(p => [p.iso2!, p])))
const paysChoisi = computed(() => (isoChoisi.value ? paysParIso.value.get(isoChoisi.value) ?? null : null))

const couleurPays = (_valeur: number, iso: string) => couleurRangPays(paysParIso.value.get(iso)?.rang)
const bullePays = (_valeur: number, iso: string) => {
  const p = paysParIso.value.get(iso)
  if (!p || p.rang == null) return 'Aucun joueur encore · cliquer pour voir'
  return `${p.rang}${p.rang === 1 ? 'er' : 'e'} · ${p.score} de score · ${p.joueurs} joueur${p.joueurs > 1 ? 's' : ''}`
}

const choisirPays = async (iso: string) => {
  isoChoisi.value = iso
  erreurPays.value = ''
  // Sous 1280 px le rail passe SOUS la carte : sans ce défilement, la fiche
  // naîtrait hors champ et rien ne dirait que le clic a produit quelque chose.
  await nextTick()
  ficheRef.value?.scrollIntoView({ behavior: 'smooth', block: 'nearest' })
}

const jouerSurPays = async ({ module, paysId }: { module: string, paysId: string }) => {
  if (!userStore.isAuthenticated) {
    redirigerVersConnexion()
    return
  }
  lancementPays.value = true
  erreurPays.value = ''
  try {
    const partie = await creerPartie(module, { paysId })
    if (partie) await navigateTo(`/activites/partie/${partie.id}`)
  }
  catch (e) {
    // Plus d'épreuve neuve sur ce pays : c'est la page du module qui annonce l'entraînement.
    erreurPays.value = messageErreurJeu(e, 'La partie n\'a pas pu être lancée.')
  }
  finally {
    lancementPays.value = false
  }
}

type Onglet = 'membres' | 'mon-pays' | 'pays'
const TAILLE = 50

const saisons = ref<SaisonsAPI | null>(null)
const saisonId = ref<string>('')
const onglet = ref<Onglet>('membres')
const page = ref(1)

const moi = ref<MonJeuAPI | null>(null)
const membres = ref<ClassementMembresAPI | null>(null)
const pays = ref<LignePaysAPI[]>([])

const chargement = ref(true)
const chargementListe = ref(false)
const erreur = ref('')

/** Saisons sélectionnables : celle en cours d'abord, puis les archives. */
const choixSaisons = computed<SaisonAPI[]>(() => [
  ...(saisons.value?.courante ? [saisons.value.courante] : []),
  ...(saisons.value?.archives ?? []),
])
const saison = computed(() => choixSaisons.value.find(s => s.id === saisonId.value) ?? null)

const onglets = computed(() => [
  { valeur: 'membres', libelle: 'Tous les membres' },
  // « Mon pays » n'a de sens que pour un membre rattaché à un pays.
  ...(moi.value?.pays_rattachement
    ? [{ valeur: 'mon-pays', libelle: moi.value.pays_rattachement.nom }]
    : []),
  { valeur: 'pays', libelle: 'Les pays' },
])

const totalPages = computed(() => Math.max(1, Math.ceil((membres.value?.total ?? 0) / TAILLE)))
const moiId = computed(() => userStore.user?.id ?? null)

/** Le membre est-il visible dans la page affichée ? Sinon, on montre sa position à part. */
const moiVisible = computed(
  () => membres.value?.elements.some(l => l.utilisateur_id === moiId.value) ?? false,
)

const dateCourte = (iso: string) =>
  new Date(iso).toLocaleDateString('fr-FR', { day: 'numeric', month: 'long', year: 'numeric' })

const chargerListe = async () => {
  if (!saisonId.value) return
  chargementListe.value = true
  erreur.value = ''
  try {
    if (onglet.value === 'pays' || vue.value === 'carte') {
      pays.value = await classementPays(saisonId.value)
    }
    else {
      membres.value = await classementMembres({
        saison: saisonId.value,
        pays: onglet.value === 'mon-pays' ? moi.value?.pays_rattachement?.id : undefined,
        page: page.value,
        taille: TAILLE,
      })
    }
  }
  catch {
    erreur.value = 'Impossible de charger ce classement pour le moment.'
  }
  finally {
    chargementListe.value = false
  }
}

onMounted(async () => {
  try {
    // « Mon jeu » n'existe que connecté ; son échec ne doit pas masquer la page.
    const [s, m] = await Promise.all([
      chargerSaisons(),
      userStore.isAuthenticated ? monJeu().catch(() => null) : Promise.resolve(null),
    ])
    saisons.value = s
    moi.value = m
    saisonId.value = choixSaisons.value[0]?.id ?? ''
    await chargerListe()
  }
  catch {
    erreur.value = 'Impossible de charger le Championship pour le moment.'
  }
  finally {
    chargement.value = false
  }
})

watch(vue, (v) => {
  // La carte lit le classement des pays : on s'y place.
  if (v === 'carte') onglet.value = 'pays'
  chargerListe()
})

watch([saisonId, onglet], () => {
  if (chargement.value) return
  page.value = 1
  chargerListe()
})
watch(page, () => { if (!chargement.value) chargerListe() })
</script>

<template>
  <NuxtLayout name="africans">
    <template #fil-ariane>
      <AfricansFilAriane :segments="[{ libelle: 'Activités', vers: '/activites' }, { libelle: 'Championship' }]" />
    </template>

    <div class="flex flex-col gap-6 pb-24">
      <header class="flex flex-wrap items-end justify-between gap-4">
        <div>
          <h1 class="text-[24px]/[1.3] font-bold text-af-encre">Championship panafricain</h1>
          <p v-if="saison" class="mt-1 text-[14px]/[1.5] text-af-corps">
            {{ saison.nom }} · du {{ dateCourte(saison.debut_at) }} au {{ dateCourte(saison.fin_at) }}
            <span v-if="saison.etat === 'close'" class="text-af-atone">(terminée)</span>
          </p>
        </div>

        <label v-if="choixSaisons.length > 1" class="flex items-center gap-2 text-[14px]/[1.4] text-af-corps">
          Saison
          <select
            v-model="saisonId"
            class="h-10 rounded-lg border border-af-bordure bg-af-surface px-3 text-[14px] text-af-encre focus-visible:outline-2 focus-visible:outline-af-chocolat"
          >
            <option v-for="s in choixSaisons" :key="s.id" :value="s.id">
              {{ s.nom }}{{ s.etat === 'en_cours' ? ' (en cours)' : '' }}
            </option>
          </select>
        </label>
      </header>

      <div v-if="chargement" class="h-80 animate-pulse rounded-[10px] bg-af-bordure" />

      <!-- Hors saison : on joue quand même, mais rien ne compte pour un classement -->
      <section
        v-else-if="choixSaisons.length === 0"
        class="rounded-[10px] border border-af-bordure bg-af-surface p-8 text-center"
      >
        <font-awesome-icon icon="fa-solid fa-trophy" class="text-4xl text-af-atone" />
        <h2 class="mt-4 text-[17px]/[1.4] font-bold text-af-encre">Aucune saison pour le moment</h2>
        <p class="mt-2 text-[14px]/[1.5] text-af-corps">
          Vous pouvez jouer : votre score s'ajoute à votre total. Il comptera pour le Championship dès
          l'ouverture de la prochaine saison.
        </p>
        <AfricansBouton class="mt-6" vers="/activites">Voir les activités</AfricansBouton>
      </section>

      <template v-else>
        <p
          v-if="saisons && !saisons.courante"
          class="rounded-[10px] border border-af-bordure bg-af-surface px-4 py-3 text-[14px]/[1.5] text-af-corps"
        >
          Aucune saison n'est en cours : ce classement est une archive.
        </p>

        <!-- Membre sans pays : il est classé parmi tous, dans aucun pays -->
        <p
          v-if="moi && !moi.pays_rattachement"
          class="flex flex-wrap items-center gap-3 rounded-[10px] border border-af-chocolat/40 bg-af-chocolat/[0.06] px-4 py-3 text-[14px]/[1.5] text-af-encre"
        >
          <font-awesome-icon icon="fa-solid fa-circle-info" class="text-af-chocolat" />
          <span class="flex-1">
            Votre profil n'indique aucun pays : vous figurez au classement de tous les membres, mais vous ne
            jouez pour aucun pays.
          </span>
          <NuxtLink to="/mon-compte/profil" class="font-bold text-af-chocolat underline-offset-2 hover:underline">
            Compléter mon profil
          </NuxtLink>
        </p>

        <div class="flex flex-wrap items-center justify-between gap-4">
          <AfricansBascule
            v-model="vue"
            libelle="Affichage"
            :options="[
              { valeur: 'classement', libelle: 'Classement', icone: 'fa-solid fa-list' },
              { valeur: 'carte', libelle: 'Carte', icone: 'fa-solid fa-earth-africa' },
            ]"
          />
        </div>

        <AfricansOnglets v-if="vue === 'classement'" v-model="onglet" :onglets="onglets" />

        <!-- Carte de l'Afrique : chaque pays teinté selon son rang -->
        <section v-if="vue === 'carte'" class="rounded-[10px] border border-af-bordure bg-af-surface p-4">
          <ClientOnly>
            <CommonCarteAfriqueValeurs
              :comptes="scoresParIso"
              :selected-iso="isoChoisi"
              :couleur="couleurPays"
              :libelle-bulle="bullePays"
              cliquable-a-zero
              @select="choisirPays"
            />
            <template #fallback>
              <div class="aspect-square animate-pulse rounded-lg bg-af-bordure" />
            </template>
          </ClientOnly>
          <p class="mt-2 text-center text-[13px]/[1.5] text-af-atone">
            Choisissez un pays pour voir son rang, ses meilleurs joueurs, et jouer sur lui.
          </p>
        </section>

        <p
          v-if="erreur"
          class="flex items-center gap-2 rounded-[10px] border border-af-live/30 bg-af-live/5 px-4 py-3 text-[14px]/[1.4] text-af-live"
          role="alert"
        >
          <font-awesome-icon icon="fa-solid fa-circle-exclamation" />
          {{ erreur }}
        </p>

        <!-- Classement des pays -->
        <section
          v-if="vue === 'classement' && onglet === 'pays'"
          class="rounded-[10px] border border-af-bordure bg-af-surface"
          :class="chargementListe && 'opacity-60'"
        >
          <p class="px-5 pt-5 pb-2 text-[13px]/[1.5] text-af-atone">
            Le score d'un pays est celui de ses meilleurs joueurs : un petit pays peut devancer un grand.
          </p>
          <ol class="divide-y divide-af-bordure">
            <li v-for="ligne in pays" :key="ligne.pays_id" class="flex items-center gap-4 px-5 py-3">
              <span
                class="w-8 shrink-0 text-center text-[15px] font-bold tabular-nums"
                :class="ligne.rang && ligne.rang <= 3 ? 'text-af-chocolat' : 'text-af-atone'"
              >{{ ligne.rang ?? '–' }}</span>
              <img
                v-if="ligne.iso2"
                :src="`https://flagcdn.com/${ligne.iso2}.svg`"
                alt=""
                class="h-5 w-7 shrink-0 rounded-sm border border-af-bordure object-cover"
                loading="lazy"
              >
              <div class="min-w-0 flex-1">
                <p class="truncate text-[15px]/[1.4] font-bold" :class="ligne.score > 0 ? 'text-af-encre' : 'text-af-atone'">
                  {{ ligne.nom }}
                </p>
                <p class="text-[13px]/[1.4] text-af-atone">
                  <template v-if="ligne.joueurs > 0">{{ ligne.joueurs }} joueur{{ ligne.joueurs > 1 ? 's' : '' }}</template>
                  <template v-else>Aucun joueur encore</template>
                </p>
              </div>
              <span class="shrink-0 text-[16px]/[1.4] font-bold tabular-nums" :class="ligne.score > 0 ? 'text-af-encre' : 'text-af-atone'">
                {{ ligne.score }}
              </span>
            </li>
          </ol>
        </section>

        <!-- Classements de membres -->
        <template v-else-if="vue === 'classement'">
          <!-- La position du membre quand elle n'est pas dans la page affichée -->
          <section
            v-if="membres?.moi && !moiVisible"
            class="rounded-[10px] border border-af-chocolat/40 bg-af-surface"
          >
            <h2 class="px-5 pt-4 pb-1 text-[14px]/[1.4] font-bold text-af-chocolat">Votre position</h2>
            <JeuTableauClassement :lignes="membres.moi.voisins" :moi-id="moiId" />
          </section>

          <section class="rounded-[10px] border border-af-bordure bg-af-surface" :class="chargementListe && 'opacity-60'">
            <JeuTableauClassement :lignes="membres?.elements ?? []" :moi-id="moiId">
              <template #vide>
                Personne n'a encore de score dans cette saison. Jouez une partie pour ouvrir le classement.
              </template>
            </JeuTableauClassement>
          </section>

          <nav v-if="totalPages > 1" class="flex items-center justify-center gap-4" aria-label="Pages du classement">
            <AfricansBouton variante="secondaire" :desactive="page <= 1" @click="page -= 1">Précédent</AfricansBouton>
            <span class="text-[14px]/[1.4] text-af-corps">Page {{ page }} sur {{ totalPages }}</span>
            <AfricansBouton variante="secondaire" :desactive="page >= totalPages" @click="page += 1">Suivant</AfricansBouton>
          </nav>
        </template>
      </template>
    </div>

    <!-- Rail de la carte : la fiche du pays choisi, et la légende -->
    <template v-if="vue === 'carte'" #rail>
      <div ref="ficheRef" class="scroll-mt-24">
        <JeuFichePaysClassement
          v-if="paysChoisi"
          :pays-id="paysChoisi.pays_id"
          :lancement="lancementPays"
          :erreur="erreurPays"
          @jouer="jouerSurPays"
        />
      </div>
      <AfricansPanneau titre="Légende" icone="fa-solid fa-ranking-star">
        <ul class="flex flex-col gap-2">
          <li v-for="palier in PALIERS_RANG" :key="palier.libelle" class="flex items-center gap-3 text-[14px]/[1.4] text-af-corps">
            <span class="size-4 shrink-0 rounded-sm" :style="{ backgroundColor: palier.couleur }" />
            {{ palier.libelle }}
          </li>
          <li class="flex items-center gap-3 text-[14px]/[1.4] text-af-corps">
            <span class="size-4 shrink-0 rounded-sm" :style="{ backgroundColor: COULEUR_SANS_JOUEUR }" />
            Aucun joueur encore
          </li>
        </ul>
      </AfricansPanneau>
    </template>
  </NuxtLayout>
</template>
